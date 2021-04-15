use serenity::{framework::standard::CommandResult, model::prelude::*, prelude::*};

pub(crate) async fn modify(ctx: &Context, msg: &Message, to_send: &str) -> CommandResult {
    let d = msg.delete(&ctx.http);
    // following match statement makes it so the new message will reply to the same message that
    // the one being deleted replied to, but only if it replied to a message.
    match &msg.referenced_message {
        Some(m) => {
            let m = m.reply(&ctx.http, &to_send);
            tokio::try_join!(d, m)?; // do both at the same time and continue once both return Ok(). It'll quit if one returns any Err()
        }
        None => {
            let m = msg.channel_id.say(&ctx.http, &to_send);
            tokio::try_join!(d, m)?;
        }
    }
    Ok(())
}
