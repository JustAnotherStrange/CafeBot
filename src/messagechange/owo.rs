// owo command for text wike twis uwu
use owoify_rs::{Owoifiable, OwoifyLevel};

use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};

use crate::messagechange::modify;

#[command]
async fn owo(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mut response = args.rest().owoify(&OwoifyLevel::Uwu); // use owoify-rs crate
    response.insert_str(0, "@: ");
    response.insert_str(1, &msg.author.name);
    modify::modify(ctx, msg, &response).await?;
    Ok(())
}
