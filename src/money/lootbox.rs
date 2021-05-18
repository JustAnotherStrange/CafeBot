use rand::{thread_rng, Rng};
use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

const ITEMS: [&str; 2] = ["Item 1", "Item 2"];

#[command]
async fn lootbox(ctx: &Context, msg: &Message) -> CommandResult {
    let won = ITEMS[thread_rng().gen_range(0..ITEMS.len())];
    msg.reply(&ctx.http, won).await?;
    Ok(())
}
