// Give money to someone else.
use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::Mentionable,
};

use crate::database::database::{get_money, money_increment};

#[command]
#[aliases("give")]
async fn give_money(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let amount = match args.single::<i32>() {
        Ok(x) => x,
        Err(_) => {
            msg.reply(
                &ctx.http,
                "Please run the command in this syntax: `^give_money [amount] [recipient]`",
            )
            .await?;
            return Ok(());
        }
    };
    if amount > get_money(&msg.author)? || amount < 0 {
        msg.reply(&ctx.http, "You can't give more money than you have.")
            .await?;
        return Ok(());
    }
    let mentions = &msg.mentions;
    if mentions.len() != 1 {
        msg.reply(
            &ctx.http,
            "Please run the command in this syntax: `^give_money [amount] [recipient]`",
        )
        .await?;
        return Ok(());
    }
    if &mentions[0] == &msg.author {
        msg.reply(&ctx.http, "You can't give money to yourself.")
            .await?;
        return Ok(());
    }
    money_increment(&mentions[0], msg.guild_id.unwrap().as_u64().clone(), amount)?;
    money_increment(&msg.author, msg.guild_id.unwrap().as_u64().clone(), -amount)?;
    let response = format!(
        "{} has received **{}** monies from {}.",
        &mentions[0].mention(),
        amount,
        &msg.author.mention()
    );
    msg.reply(&ctx.http, response).await?;
    Ok(())
}
