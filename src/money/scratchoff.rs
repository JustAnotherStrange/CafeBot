// use scratchoff tickets that are purchased in the shop
use crate::database::database::{gen_connection, get_so, money_increment_with_lost};
use crate::money::blackjack::edit_embed;
use crate::tools::help::EditContent;
use rand::{thread_rng, Rng};
use rusqlite::params;
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    prelude::*,
};
use std::time::Duration;

#[command]
#[aliases("so")]
async fn scratchoff(ctx: &Context, msg: &Message) -> CommandResult {
    let conn = gen_connection();
    let user_so = get_so(&msg.author, &conn);
    // needed in order for embed to be a reply
    let mut message = msg
        .reply(&ctx.http, "**Welcome to CafeBot Scratch-Off!**")
        .await?;
    edit_embed(&ctx, &mut message, "Loading...", "").await;
    // initial reactions
    let emojis: Vec<char> = vec!['🇦', '🇧', '🇨', '🛑'];
    for emoji in emojis.iter() {
        message.react(&ctx, *emoji).await?;
    }
    // initial embed message
    let desc = format!(
        "Please pick which tier you want to scratch off!
    A - Tier 1 (You have {})
    B - Tier 2 (You have {})
    C - Tier 3 (You have {})

    You can buy more scratch-off tickets in the shop using `^shop`.",
        user_so.tier1, user_so.tier2, user_so.tier3
    );
    let default_response = EditContent {
        title: "__CafeBot Scratch-Off__".to_string(),
        description: desc,
    };
    edit_embed(
        &ctx,
        &mut message,
        default_response.title.as_str(),
        default_response.description.as_str(),
    )
    .await;

    // Reaction awaiting loop
    let tier: i32 = 'main: loop {
        if let Some(reaction) = message
            .await_reaction(&ctx)
            .timeout(Duration::from_secs(60)) // after 120 seconds without reactions, it will go to the "else" statement.
            .await
        {
            let emoji = &reaction.as_inner_ref().emoji;
            let reacted = &*reaction.as_inner_ref().clone();
            if reacted.user(&ctx).await? != msg.author {
                continue 'main; // make it so only the author can use the tickets
            }
            let mut response = default_response.clone();
            // match on the reacted emoji
            // these breaks set the tier value to the number. didn't know you could do that until recently :o
            match emoji.as_data().as_str() {
                "🇦" => {
                    // tier 1
                    break 1;
                }
                "🇧" => {
                    // tier 2
                    break 2;
                }
                "🇨" => {
                    // tier 3
                    break 3;
                }
                "🛑" => {
                    response = EditContent {
                        title: "Goodbye!".to_string(),
                        description: "This message is inactive.".to_string(),
                    };
                    edit_embed(
                        &ctx,
                        &mut message,
                        response.title.as_str(),
                        response.description.as_str(),
                    )
                    .await;
                    return Ok(()); // end
                }
                _ => {} // if the reaction is none of the above, then do nothing (loops around and awaits another reaction).
            }
            edit_embed(
                &ctx,
                &mut message,
                response.title.as_str(),
                response.description.as_str(),
            )
            .await;
        } else {
            // gets here if there were no reactions for 120 seconds.
            edit_embed(ctx, &mut message, "Goodbye!", "This message is inactive.").await;
            return Ok(()); // end
        }
    };
    let so_amnt = match tier {
        1 => user_so.tier1,
        2 => user_so.tier2,
        3 => user_so.tier3,
        _ => unreachable!(), // if the if statement in the let tier worked then this should be unreachable
    };
    if so_amnt == 0 {
        edit_embed(
            &ctx,
            &mut message,
            "You don't have any of that ticket tier!",
            "You can buy some in the shop using `^shop`.",
        )
        .await;
        return Ok(());
    }

    // finally, all the checks are done. reply to start the embedded message
    // generate amount of money. different tiers will just be multipliers on this amount.
    let r = thread_rng().gen_range(0..101);
    // gen weighted win amount
    // I feel as if it is weighted to be too low, but I don't want the person's amount of money to trend up or down.
    let mut win_amount: i32;
    win_amount = match r {
        0..=33 => thread_rng().gen_range(0..100),
        0..=67 => thread_rng().gen_range(100..200),
        68..=89 => thread_rng().gen_range(200..300),
        90..=96 => thread_rng().gen_range(300..350),
        97..=99 => thread_rng().gen_range(350..400),
        100 => 500,
        _ => unreachable!(), // shouldnt be possible
    };
    win_amount = match tier {
        1 => win_amount,
        2 => win_amount * 2,
        3 => win_amount * 4,
        _ => unreachable!(), // same situation as the previous unreachable.
    };

    // take away the ticket they used
    match tier {
        1 => conn.execute(
            "update users set so_tier1 = so_tier1 - 1 where id = ?1",
            params![&msg.author.id.as_u64()],
        )?,
        2 => conn.execute(
            "update users set so_tier2 = so_tier2 - 1 where id = ?1",
            params![&msg.author.id.as_u64()],
        )?,
        3 => conn.execute(
            "update users set so_tier3 = so_tier3 - 1 where id = ?1",
            params![&msg.author.id.as_u64()],
        )?,
        _ => return Ok(()), // this should never be reached, but unreachable!() complains about return types of match arms
    };
    // give user the money they won
    money_increment_with_lost(
        &msg.author,
        msg.guild_id.unwrap().as_u64().clone(),
        win_amount,
    )?;
    let description = format!(
        "Your **Tier {} Scratch-Off Ticket** won you **{}** monies!",
        tier, win_amount
    );
    edit_embed(&ctx, &mut message, "Congratulations!", description.as_str()).await;
    Ok(())
}
