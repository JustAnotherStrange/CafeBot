use crate::database::database::get_lost;
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::prelude::*,
    prelude::*,
};

#[command]
#[only_in(guilds)]
async fn stats(ctx: &Context, msg: &Message) -> CommandResult {
    let name = msg.guild_id.unwrap().name(&ctx).await.unwrap();
    let lost_amount = get_lost(*msg.guild_id.unwrap().as_u64()).unwrap();
    let response = format!(
        "Your server, **{}**, has lost a total of of **{}** monies.",
        name, lost_amount
    );
    msg.reply(&ctx.http, response).await?;
    Ok(())
}
