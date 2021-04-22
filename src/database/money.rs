use crate::database::database::{create_user_if_not_exist, get_money, money_increment};
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::prelude::*,
    prelude::*,
};

#[command]
async fn money_up(ctx: &Context, msg: &Message) -> CommandResult {
    create_user_if_not_exist(msg)?;
    money_increment(msg, 1)?;
    let to_send = format!("Money up! You now have **{}** monies.", get_money(msg)?);
    msg.reply(&ctx.http, to_send).await?;
    Ok(())
}

#[command]
#[aliases("wallet")]
async fn money(ctx: &Context, msg: &Message) -> CommandResult {
    create_user_if_not_exist(msg)?;
    let to_send = format!("You have **{}** monies", get_money(msg)?);
    msg.reply(&ctx.http, to_send).await?;
    Ok(())
}
