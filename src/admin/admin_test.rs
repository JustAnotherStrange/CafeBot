// tests if the user running is an admin
use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::{channel::Message, prelude::*},
};

// https://github.com/serenity-rs/serenity/blob/dcc1ac4d0a12f24e998af3949e33ec352153a6af/examples/e05_command_framework/src/main.rs#L522
#[command]
#[only_in(guilds)]
async fn admin_test(ctx: &Context, msg: &Message) -> CommandResult {
    if is_admin(ctx, msg).await {
        msg.reply(&ctx.http, "You are an admin.").await?;
    } else {
        msg.reply(&ctx.http, "You are not an admin.").await?;
    }
    Ok(())
}

pub async fn is_admin(ctx: &Context, msg: &Message) -> bool {
    if let Some(member) = &msg.member {
        for role in &member.roles {
            if role
                .to_role_cached(&ctx.cache)
                .await
                .map_or(false, |r| r.has_permission(Permissions::ADMINISTRATOR))
            {
                return true;
            }
        }
    }
    return false;
}
