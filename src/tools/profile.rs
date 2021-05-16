use crate::database::database::{
    create_user_if_not_exist, gen_connection, get_incr_amount, get_money, get_so,
};
use crate::money::daily::get_daily_streak;
use serenity::{
    framework::standard::{macros::command, CommandResult},
    http::AttachmentType,
    model::prelude::*,
    prelude::*,
};

#[command]
#[only_in(guilds)]
async fn profile(ctx: &Context, msg: &Message) -> CommandResult {
    // let you check profile of other users
    let mentions = &msg.mentions;
    let user = match mentions.len() {
        0 => &msg.author,
        1 => &mentions[0],
        _ => {
            msg.reply(&ctx.http,"Please enter either one user that you want to see the profile of, or enter no user to see your own profile.")
                .await?;
            return Ok(());
        }
    };
    let conn = gen_connection();
    create_user_if_not_exist(&user, &conn).unwrap();
    let money = get_money(user).unwrap();
    let incr_amount = get_incr_amount(user, &conn);

    // bugfix
    let streak = match get_daily_streak(user) {
        Some(x) => x,
        None => 0,
    };
    let so = get_so(user, &conn);

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
        "Monies: **{}**
        Idle increase: **{}**
        Daily streak: **{}**
        Tier 1 Scratch-Offs: **{}**
        Tier 2 Scratch-Offs: **{}**
        Tier 3 Scratch-Offs: **{}**",
        money, incr_amount, streak, so.tier1, so.tier2, so.tier3
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
