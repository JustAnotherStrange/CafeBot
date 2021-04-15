use chrono::Utc;
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::prelude::*,
    prelude::*,
};

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
