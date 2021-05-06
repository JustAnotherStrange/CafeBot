// use scratchoff tickets that are purchased in the shop
// todo: there will be different tiers of scratch-offs, lose or gain more money.
// todo: make new table in db: tickets, with a column for user id, tier 1 tickets, tier 2 tickets, ...
use crate::database::database::{get_money, money_increment};
use rand::{thread_rng, Rng};
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::*,
    Error,
};
use std::{thread::sleep, time, time::Duration};

#[command]
async fn scratchoff(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    // parse bet amount
    let bet: i32 = match args.rest().trim().parse() {
        Ok(x) => x,
        Err(_) => {
            msg.reply(&ctx.http, "Please enter an amount to bet as an argument.")
                .await?;
            return Ok(());
        }
    };
    if bet > get_money(&msg.author)? || bet < 0 {
        msg.reply(&ctx.http, "You can't bet more money than you have.")
            .await?;
        return Ok(());
    }
    // generate amount of money. different tiers will just be multipliers on this amount.
    let r = thread_rng().gen_range(0..101);
    // gen weighted win amount
    let win_amount: i32;
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
        win_amount = 4000;
    };
    msg.reply(&ctx.http, win_amount).await?;
    Ok(())
}
