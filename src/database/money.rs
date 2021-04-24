use crate::database::database::{get_money, money_increment};
use rand::{thread_rng, Rng};
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};

// // money_up command originally used for testing the database.
// #[command]
// async fn money_up(ctx: &Context, msg: &Message) -> CommandResult {
//     money_increment(msg, 1)?;
//     let to_send = format!("Money up! You now have **{}** monies.", get_money(msg)?);
//     msg.reply(&ctx.http, to_send).await?;
//     Ok(())
// }

#[command]
async fn coin_flip(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let bet: i32 = match args.rest().trim().parse() {
        Ok(x) => x,
        Err(_) => {
            msg.reply(&ctx.http, "Please enter an amount to bet as an argument.")
                .await?;
            return Ok(());
        }
    };
    if bet > get_money(&msg.author)? {
        msg.reply(&ctx.http, "You can't bet more money than you have.")
            .await?;
        return Ok(());
    }
    if thread_rng().gen_bool(0.5) {
        money_increment(&msg.author, bet)?;
        let response = format!("You got heads! You got **{}** monies.", bet);
        msg.reply(&ctx.http, response).await?;
    } else {
        money_increment(&msg.author, -bet)?;
        let response = format!("You got tails! You lost **{}** monies.", bet);
        msg.reply(&ctx.http, response).await?;
    }
    Ok(())
}

#[command]
#[aliases("give")]
async fn give_money(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let amount = match args.single::<i32>() {
        Ok(x) => x,
        Err(_) => {
            msg.reply(
                &ctx.http,
                "Please run the command in this syntax: `^give_money [amount] [recipient]`",
            )
            .await?;
            return Ok(());
        }
    };
    if amount > get_money(&msg.author)? || amount < 0 {
        msg.reply(&ctx.http, "You can't give more money than you have.")
            .await?;
        return Ok(());
    }
    let mentions = &msg.mentions;
    if mentions.len() != 1 {
        msg.reply(
            &ctx.http,
            "Please run the command in this syntax: `^give_money [amount] [recipient]`",
        )
        .await?;
        return Ok(());
    }
    if &mentions[0] == &msg.author {
        msg.reply(&ctx.http, "You can't give money to yourself.")
            .await?;
        return Ok(());
    }
    money_increment(&mentions[0], amount)?;
    money_increment(&msg.author, -amount)?;
    let response = format!(
        "{} has received **{}** monies from {}.",
        &mentions[0].mention(),
        amount,
        &msg.author.mention()
    );
    msg.reply(&ctx.http, response).await?;
    Ok(())
}

#[command]
#[aliases("wallet")]
async fn money(ctx: &Context, msg: &Message) -> CommandResult {
    let to_send = format!("You have **{}** monies.", get_money(&msg.author)?);
    msg.reply(&ctx.http, to_send).await?;
    Ok(())
}
