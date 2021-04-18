// sets slow mode
use crate::is_admin;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};

#[command]
#[only_in(guilds)]
// Only people with administrator permissions can run
// #[required_permissions(ADMINISTRATOR)]
// taken from example on serenity github
// https://github.com/serenity-rs/serenity/blob/dcc1ac4d0a12f24e998af3949e33ec352153a6af/examples/e05_command_framework/src/main.rs#L540
async fn slow_mode(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if is_admin(ctx, msg).await {
        // if you have reached here, you are admin. now do the command.
        let say_content = if let Ok(slow_mode_rate_seconds) = args.single::<u64>() {
            if let Err(why) = msg
                .channel_id
                .edit(&ctx.http, |c| c.slow_mode_rate(slow_mode_rate_seconds))
                .await
            {
                format!(
                    "Failed to set slow mode to `{}` seconds. because {}",
                    slow_mode_rate_seconds, why
                )
            } else {
                format!(
                    "Successfully set slow mode rate to `{}` seconds.",
                    slow_mode_rate_seconds
                )
            }
        } else if let Some(Channel::Guild(channel)) =
            msg.channel_id.to_channel_cached(&ctx.cache).await
        {
            format!(
                "Current slow mode rate is `{}` seconds.",
                channel.slow_mode_rate.unwrap_or(0)
            )
        } else {
            "Failed to find channel in cache.".to_string()
        };
        msg.reply(&ctx.http, say_content).await?;
        return Ok(());
    }
    msg.reply(&ctx.http, "You can't run that command.").await?;
    Ok(())
}
