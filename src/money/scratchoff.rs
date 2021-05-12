// use scratchoff tickets that are purchased in the shop
// todo: there will be different tiers of scratch-offs, lose or gain more money.
// todo: make new table in db: tickets, with a column for user id, tier 1 tickets, tier 2 tickets, ...
use crate::database::database::{gen_connection, get_money, get_so, money_increment};
use crate::money::blackjack::edit_embed;
use crate::tools::help::EditContent;
use rand::{thread_rng, Rng};
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::*,
};
use std::time::Duration;

#[command]
async fn scratchoff(ctx: &Context, msg: &Message) -> CommandResult {
    let conn = gen_connection();

    // needed in order for embed to be a reply
    let mut message = msg.reply(&ctx.http, "**Welcome to CafeBot!**").await?;
    // initial reactions
    let emojis: Vec<char> = vec!['🇦', '🇧', '🇨', '🛑'];
    for emoji in emojis.iter() {
        message.react(&ctx, *emoji).await?;
    }
    // initial embed message
    edit_embed(
        &ctx,
        &mut message,
        "__CafeBot Scratch-Off__",
        "Please pick which tier you want to scratch off! Buy in the `^shop` and check how many you have in `^profile`.",
    )
    .await;

    // Reaction awaiting loop
    let tier: i32 = '_main: loop {
        if let Some(reaction) = message
            .await_reaction(&ctx)
            .timeout(Duration::from_secs(60)) // after 120 seconds without reactions, it will go to the "else" statement.
            .await
        {
            let emoji = &reaction.as_inner_ref().emoji;
            let mut response = EditContent {
                title: "Pick which tier you want!
                A - Tier 1
                B - Tier 2
                C - Tier 3"
                    .to_string(),
                description: "".to_string(),
            };
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
    let user_so = get_so(&msg.author, &conn);
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
            "Nice try.",
        )
        .await;
        return Ok(());
    }

    // finally, all the checks are done. reply to start the embedded message
    // generate amount of money. different tiers will just be multipliers on this amount.
    let r = thread_rng().gen_range(0..101);
    // gen weighted win amount
    let mut win_amount: i32;
    if r >= 0 && r <= 50 {
        win_amount = thread_rng().gen_range(0..50);
    } else if r >= 51 && r <= 75 {
        win_amount = thread_rng().gen_range(50..100);
    } else if r >= 76 && r <= 85 {
        win_amount = thread_rng().gen_range(100..150);
    } else if r >= 86 && r <= 92 {
        win_amount = thread_rng().gen_range(150..200);
    } else if r >= 93 && r <= 95 {
        win_amount = thread_rng().gen_range(200..250);
    } else if r >= 96 && r <= 98 {
        win_amount = thread_rng().gen_range(250..300);
    } else if r == 99 {
        win_amount = thread_rng().gen_range(300..350);
    } else {
        win_amount = 400;
    };
    win_amount = match tier {
        1 => win_amount,
        2 => win_amount * 2,
        3 => win_amount * 4,
        _ => unreachable!(), // same situation as the previous unreachable.
    };
    let description = format!(
        "Your **Tier {} Scratch-Off Ticket** won you **{}** monies!",
        tier, win_amount
    );
    edit_embed(&ctx, &mut message, "Congratulations!", description.as_str()).await;
    Ok(())
}
