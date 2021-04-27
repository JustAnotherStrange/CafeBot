use crate::database::database::{create_user_if_not_exist, gen_connection, get_money};
use crate::fun::blackjack::edit_embed;
use rusqlite::{params, Connection};
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::{channel::Message, user::User},
    prelude::*,
};
use std::time::Duration;

#[command]
async fn shop(ctx: &Context, msg: &Message) -> CommandResult {
    let conn = gen_connection();
    // reply a message first and edit it with the embed, as a workaround to make the embed message be a reply
    let response = format!("**{}** entered the shop!", &msg.author.name);
    let mut message = msg.reply(&ctx.http, &response).await?;

    // initial embed
    message
        .edit(&ctx, |m| {
            m.embed(|e| {
                e.title("Loading...");
                e
            })
        })
        .await?;
    let letters: Vec<char> = vec!['🎫', '🛑']; // ticket
    for letter in letters.iter() {
        message.react(ctx, *letter).await?;
    }
    let ticket_amnt = get_amount_of_tickets(&msg.author, &conn)?;
    let ticket_price: u32 = 100 * (2_u32.pow(ticket_amnt));
    let description = format!(
        "{}: Ticket: {} monies\n{}: Leave the shop.",
        letters[0], ticket_price, letters[1]
    );
    edit_embed(
        &ctx,
        &mut message,
        "Welcome to the CafeBot Shop!",
        &*description,
    )
    .await;
    'main: loop {
        if let Some(reaction) = message
            .await_reaction(&ctx)
            .timeout(Duration::from_secs(120)) // after 120 seconds without reactions, it will go to the "else" statement.
            .await
        {
            let emoji = &reaction.as_inner_ref().emoji;
            let reacted = &*reaction.as_inner_ref().clone();
            if reacted.user(&ctx).await? != msg.author {
                continue 'main;
            }
            // match on the reacted emoji
            match emoji.as_data().as_str() {
                "🎫" => {
                    if get_amount_of_tickets(&msg.author, &conn)? >= 5 {
                        edit_embed(
                            &ctx,
                            &mut message,
                            "You have hit the maximum amount of tickets.",
                            "Please wait until the next drawing to purchase more tickets.",
                        )
                            .await;
                        return Ok(());
                    }
                    match purchase(&msg.author, ticket_price).await {
                        Ok(_) => {
                            conn.execute(
                                "update users set tickets = tickets + 1 where id = ?1",
                                params![msg.author.id.as_u64()],
                            )?;
                            let description = format!(
                                "You purchased a **ticket** for **{}** monies.",
                                ticket_price
                            );
                            edit_embed(&ctx, &mut message, "Success!", &*description).await;
                        }
                        Err(_) => {
                            edit_embed(
                                &ctx,
                                &mut message,
                                "Nice try, but you don't have enough money to buy that.",
                                "haha poor.",
                            )
                            .await;
                        }
                    };
                    return Ok(());
                }
                "🛑" => {
                    edit_embed(&ctx, &mut message, "Goodbye!", "The shop is closed here.").await;
                    return Ok(());
                }
                _ => {} // if the reaction is none of the above, then do nothing.
            }
        } else {
            // gets here if there were no reactions for 120 seconds.
            let new_description = "Two minutes passed with no reactions, so the shop closed.";
            edit_embed(ctx, &mut message, "Timed out.", new_description).await;
            return Ok(()); // close the shop
        }
    }
    // Ok(())
}

pub fn get_amount_of_tickets(user: &User, conn: &Connection) -> Result<u32, rusqlite::Error> {
    create_user_if_not_exist(&user, &conn).unwrap();
    // let mut stmt = conn.prepare("select money from users where id = ?1")?;
    let money = conn.query_row(
        "select tickets from users where id = ?1",
        params![user.id.as_u64()],
        |row| Ok(row.get(0)?),
    );
    return money;
}

async fn purchase(user: &User, price: u32) -> Result<(), ()> {
    let conn = gen_connection();
    create_user_if_not_exist(&user, &conn).unwrap();
    return if price > get_money(user).unwrap() as u32 {
        Err(())
    } else {
        conn.execute(
            "update users set money = money - ?1 where id = ?2",
            params![price, user.id.as_u64()],
        )
        .unwrap();
        Ok(())
    };
}
