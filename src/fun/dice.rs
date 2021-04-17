use cute_dnd_dice::Roll;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};

#[command]
#[aliases("d")]
async fn dice(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let arguments = args.rest();
    let dice = match Roll::from_str(arguments) {
        Ok(x) => x,
        Err(_) => {
            let to_send = format!("*{}* is not a valid dice roll.", arguments);
            msg.reply(&ctx.http, &to_send).await?;
            return Ok(());
        }
    };
    let to_send = format!("Rolling *{}...*\nResults: **{}**", arguments, dice.roll());
    msg.reply(&ctx.http, &to_send).await?;
    Ok(())
}
