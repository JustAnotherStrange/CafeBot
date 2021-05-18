use crate::{
    database::database::{create_user_if_not_exist, gen_connection, get_incr_amount, get_money},
    money::blackjack::edit_embed,
};
use rusqlite::params;
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::{channel::Message, user::User},
    prelude::*,
};
use std::time::Duration;

#[command]
async fn shop(ctx: &Context, msg: &Message) -> CommandResult {
    let conn = gen_connection();
    create_user_if_not_exist(&msg.author, &conn)?;
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
    // tier reactions
    let letters: Vec<char> = vec!['👎', '✊', '👍', '📈', '🛑']; // ticket

    for letter in letters.iter() {
        message.react(ctx, *letter).await?;
    }
    let incr_amnt = get_incr_amount(&msg.author, &conn);
    let incr_price = 10 * incr_amnt.pow(2) + 200;
    let tier1_price = 200;
    let tier2_price = 400;
    let tier3_price = 800;
    let description = format!(
        "{}: Scratch-Off Tier 1: {} monies
        {}: Scratch-Off Tier 2: {} monies
        {}: Scratch-Off Tier 3: {} monies
        {}: Increase hourly increase of money by 2: {} monies
        {}: Leave the shop.",
        letters[0],
        tier1_price,
        letters[1],
        tier2_price,
        letters[2],
        tier3_price,
        letters[3],
        incr_price,
        letters[4]
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
                "👎" => {
                    match purchase(&msg.author, tier1_price as u32).await {
                        Ok(_) => {
                            conn.execute(
                                "update users set so_tier1 = so_tier1 + 1 where id = ?1",
                                params![msg.author.id.as_u64()],
                            )?;
                            let description = format!(
                                "You bought a Tier 1 Scratch-Off Ticket for **{}** monies. Use it with `^scratchoff`.",
                                tier1_price
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
                    break 'main;
                }

                "✊" => {
                    match purchase(&msg.author, tier2_price as u32).await {
                        Ok(_) => {
                            conn.execute(
                                "update users set so_tier2 = so_tier2 + 1 where id = ?1",
                                params![msg.author.id.as_u64()],
                            )?;
                            let description = format!(
                                "You bought a Tier 2 Scratch-Off Ticket for **{}** monies. Use it with `^scratchoff`.",
                                tier2_price
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
                    break 'main;
                }

                "👍" => {
                    match purchase(&msg.author, tier3_price as u32).await {
                        Ok(_) => {
                            conn.execute(
                                "update users set so_tier3 = so_tier3 + 1 where id = ?1",
                                params![msg.author.id.as_u64()],
                            )?;
                            let description = format!(
                                "You bought a Tier 3 Scratch-Off Ticket for **{}** monies. Use it with `^scratchoff`.",
                                tier3_price
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
                    break 'main;
                }

                "📈" => {
                    match purchase(&msg.author, incr_price as u32).await {
                        Ok(_) => {
                            if get_incr_amount(&msg.author, &conn) >= 50 {
                                edit_embed(
                                    &ctx,
                                    &mut message,
                                    "Failure.",
                                    "You have hit the max increase amount.",
                                )
                                .await;
                                return Ok(());
                            }
                            conn.execute(
                                "update users set incr_amount = incr_amount + 2 where id = ?1",
                                params![msg.author.id.as_u64()],
                            )?;
                            let description = format!(
                                "You increased your idle money increase by 2 for **{}** monies.",
                                incr_price
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
                    break 'main;
                }

                "🛑" => {
                    edit_embed(&ctx, &mut message, "Goodbye!", "The shop is closed here.").await;
                    // return Ok(());
                    break 'main;
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
    Ok(())
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
