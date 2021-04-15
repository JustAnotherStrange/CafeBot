use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

#[command]
// ping pong command (used mostly for checking if bot is online)
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(&ctx.http, "pong").await?;
    Ok(())
}
