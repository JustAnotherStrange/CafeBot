use crate::database::database::get_pool;
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::prelude::*,
    prelude::*,
};

#[command]
#[only_in(guilds)]
async fn pool(ctx: &Context, msg: &Message) -> CommandResult {
    let name = msg.guild_id.unwrap().name(&ctx).await.unwrap();
    let pool_amount = get_pool(*msg.guild_id.unwrap().as_u64()).unwrap();
    let response = format!(
        "Your server, **{}**, has a pool of **{}** monies.",
        name, pool_amount
    );
    msg.reply(&ctx.http, response).await?;
    Ok(())
}
