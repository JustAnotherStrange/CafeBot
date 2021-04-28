use crate::database::database::{gen_connection, get_incr_amount, get_money};
use crate::money::shop::get_amount_of_tickets;
pub use serenity::{
    framework::standard::{macros::command, CommandResult},
    http::AttachmentType,
    model::prelude::*,
    prelude::*,
};

#[command]
#[only_in(guilds)]
async fn profile(ctx: &Context, msg: &Message) -> CommandResult {
    let conn = gen_connection();
    let user = &msg.author;
    let money = get_money(user).unwrap();
    let tickets = get_amount_of_tickets(user, &conn).unwrap();
    let incr_amount = get_incr_amount(user, &conn);

    // if there is a pfp, use that. else, use the default pfp thing.
    let mut pfp_link = user.face();
    pfp_link.push_str("?size=16"); // for a smaller image
    let mut username = user.name.clone();
    match user.nick_in(&ctx, msg.guild_id.unwrap()).await {
        Some(x) => {
            let str = format!("\n\"{}\"", x);
            username.push_str(str.as_str())
        }
        None => {}
    }
    let desc = format!(
        "Monies: **{}**\nTickets: **{}**\nIdle increase: **{}**",
        money, tickets, incr_amount
    );

    // send in embed
    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title(&username);
                e.description(&desc);
                e.image("attachment://&image_link");
                e
            });
            m.add_file(AttachmentType::Image(&pfp_link));
            m
        })
        .await
        .unwrap();
    Ok(())
}
