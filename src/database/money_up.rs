use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::prelude::*,
    prelude::*,
};
use crate::database::database::{create_user_if_not_exist, money_increment, get_money, db_init};

#[command]
async fn money_up(ctx: &Context, msg: &Message) -> CommandResult {
    db_init().await.unwrap();
    create_user_if_not_exist(msg).unwrap();
    money_increment(msg, 1).unwrap();
    let to_send = format!("Money is now {}", get_money(msg).unwrap());
    msg.reply(&ctx.http, to_send).await?;
    Ok(())
}
