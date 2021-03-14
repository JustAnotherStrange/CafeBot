use serenity::{
    prelude::*,
    model::prelude::*,
    framework::standard::{
        CommandResult,
        macros::command,
    },
};
use chrono::Utc;
#[command]
// ping pong command (used mostly for checking if bot is online)
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(&ctx.http, "pong").await?;
    Ok(())
}

#[command]
async fn latency(ctx: &Context, msg: &Message) -> CommandResult {
    let sub: chrono::Duration = Utc::now() - msg.timestamp;
    msg.reply(
        &ctx.http,
        format!("latency is {} milliseconds", sub.num_milliseconds()),
    )
    .await?;
    Ok(())
}
