// admin command: changes bot status. Has as "Playing [input]"
use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::{channel::Message, gateway::Activity, prelude::*},
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
