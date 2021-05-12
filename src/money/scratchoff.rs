// use scratchoff tickets that are purchased in the shop
// todo: probabilities
use crate::database::database::{gen_connection, get_money, get_so, money_increment};
use crate::money::blackjack::edit_embed;
use crate::tools::help::EditContent;
use rand::{thread_rng, Rng};
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    prelude::*,
};
use std::time::Duration;

#[command]
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
    let desc = format!("Please pick which tier you want to scratch off!
    A - Tier 1 (You have {})
    B - Tier 2 (You have {})
    C - Tier 3 (You have {})

    You can buy more scratch-off tickets in the shop using `^shop`.",
    user_so.tier1, user_so.tier2, user_so.tier3);
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
    let tier: i32 = '_main: loop {
        if let Some(reaction) = message
            .await_reaction(&ctx)
            .timeout(Duration::from_secs(60)) // after 120 seconds without reactions, it will go to the "else" statement.
            .await
        {
            let emoji = &reaction.as_inner_ref().emoji;
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
            "Nice try.",
        )
        .await;
        return Ok(());
    }

    // finally, all the checks are done. reply to start the embedded message
    // generate amount of money. different tiers will just be multipliers on this amount.
    let r = thread_rng().gen_range(0..101);
    // gen weighted win amount
    // todo: probabilities.
    // I feel as if it is weighted to be too low, but I don't want the person's amount of money to trend up or down.
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
    // todo: make you lose a ticket when you scratch it off and make you actually win the money
    Ok(())
}
