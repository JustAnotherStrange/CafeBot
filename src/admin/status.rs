// admin command: changes bot status. Has as "Playing [input]"
use crate::is_admin;
use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::{channel::Message, gateway::Activity},
    utils::MessageBuilder,
};

#[command]
async fn status(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if is_admin(ctx, msg).await {
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
    msg.reply(&ctx.http, "You can't run that command.").await?;
    Ok(())
}
