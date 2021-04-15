// prints a random bruh emote
use rand::{thread_rng, Rng};
use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

#[command]
async fn bruh(ctx: &Context, msg: &Message) -> CommandResult {
    let choice = thread_rng().gen_range(1..5);
    match choice {
        1 => msg.reply(&ctx.http, "<:CyberBruh:702876356287135864>"),
        2 => msg.reply(&ctx.http, "<:burh:721124449252016158>"),
        3 => msg.reply(&ctx.http, "<:certifiedbruhmoment:704060742034522213>"),
        4 => msg.reply(&ctx.http, "<:bruh100:679483886241185823>"),
        _ => unreachable!(),
    }
    .await?;
    Ok(())
}
