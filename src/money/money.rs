// See how much money you have.
use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

use crate::database::database::get_money;

#[command]
#[aliases("wallet")]
async fn money(ctx: &Context, msg: &Message) -> CommandResult {
    let to_send = format!("You have **{}** monies.", get_money(&msg.author)?);
    msg.reply(&ctx.http, to_send).await?;
    Ok(())
}
