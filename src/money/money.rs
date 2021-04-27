// See how much money you have.
use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

use crate::database::database::{gen_connection, get_money};
use crate::money::shop::get_amount_of_tickets;

#[command]
#[aliases("wallet")]
async fn money(ctx: &Context, msg: &Message) -> CommandResult {
    let conn = gen_connection();
    let to_send = format!(
        "You have **{}** monies and **{}** tickets.",
        get_money(&msg.author)?,
        get_amount_of_tickets(&msg.author, &conn).unwrap()
    );
    msg.reply(&ctx.http, to_send).await?;
    Ok(())
}
