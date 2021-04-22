use crate::database::database::{create_user_if_not_exist, get_money, money_increment};
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
    if bet > get_money(msg)? {
        msg.reply(&ctx.http, "You can't bet more money than you have.")
            .await?;
        return Ok(());
    }
    if thread_rng().gen_bool(0.5) {
        money_increment(msg, bet)?;
        let response = format!("You got heads! You got **{}** monies.", bet);
        msg.reply(&ctx.http, response).await?;
    } else {
        money_increment(msg, -bet)?;
        let response = format!("You got tails! You lost **{}** monies.", bet);
        msg.reply(&ctx.http, response).await?;
    }
    Ok(())
}

#[command]
#[aliases("wallet")]
async fn money(ctx: &Context, msg: &Message) -> CommandResult {
    let to_send = format!("You have **{}** monies.", get_money(msg)?);
    msg.reply(&ctx.http, to_send).await?;
    Ok(())
}
