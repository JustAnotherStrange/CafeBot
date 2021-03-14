// is there a way to make it so every command in this file only usable by admin perms?
use serenity::{
    prelude::*,
    model::prelude::*,
    framework::standard::{
        CommandResult,
        macros::command,
        Args,
    },
    utils::MessageBuilder,
};

#[command]
async fn status(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if let Some(member) = &msg.member {
        for role in &member.roles {
            if role
                .to_role_cached(&ctx.cache)
                .await
                .map_or(false, |r| r.has_permission(Permissions::ADMINISTRATOR))
            {
                let name = args.message();
                ctx.set_activity(Activity::playing(&name)).await;
                let response = MessageBuilder::new()
                    .push("Status has been set to ")
                    .push_bold_safe("Playing")
                    .push(" ")
                    .push_bold_safe(&name)
                    .build();
                msg.reply(&ctx.http, &response).await?;
                return Ok(());
            }
        }
    }
    msg.reply(&ctx.http, "You can't run that command.").await?;
    Ok(())
}

// https://github.com/serenity-rs/serenity/blob/dcc1ac4d0a12f24e998af3949e33ec352153a6af/examples/e05_command_framework/src/main.rs#L522
#[command]
#[only_in(guilds)]
async fn admin_test(ctx: &Context, msg: &Message) -> CommandResult {
    if let Some(member) = &msg.member {
        for role in &member.roles {
            if role
                .to_role_cached(&ctx.cache)
                .await
                .map_or(false, |r| r.has_permission(Permissions::ADMINISTRATOR))
            {
                msg.reply(&ctx.http, "You are an admin.").await?;
                return Ok(());
            }
        }
    }
    msg.reply(&ctx.http, "You are not an admin.").await?;
    Ok(())
}
#[command]
#[only_in(guilds)]
// Only people with administrator permissions can run
// #[required_permissions(ADMINISTRATOR)]
// taken from example on serenity github
// https://github.com/serenity-rs/serenity/blob/dcc1ac4d0a12f24e998af3949e33ec352153a6af/examples/e05_command_framework/src/main.rs#L540
async fn slow_mode(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if let Some(member) = &msg.member {
        for role in &member.roles {
            if role
                .to_role_cached(&ctx.cache)
                .await
                .map_or(false, |r| r.has_permission(Permissions::ADMINISTRATOR))
            {
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
        }
    }
    msg.reply(&ctx.http, "You can't run that command.").await?;
    Ok(())
}
