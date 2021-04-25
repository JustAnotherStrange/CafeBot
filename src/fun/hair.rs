// baldness calculator (actually just a random number generator).
// You can also specify who to test (e.g. ^bald @joe)
use rand::{thread_rng, Rng};
use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    utils::MessageBuilder,
};

#[command]
#[aliases("bald")]
async fn hair(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let hairlevel = thread_rng().gen_range(0..101);
    let args_string = args.rest();
    let response = MessageBuilder::new()
        .push_bold_safe(if args_string == "" {
            &msg.author.name
        } else {
            args_string
        }) // use the arguments for the person to be tested
        .push(" has ")
        .push_bold_safe(&hairlevel)
        .push("% hair.")
        .build();
    msg.reply(&ctx.http, &response).await?;
    Ok(())
}

#[command]
async fn balder(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let hairlevel: f64 = (thread_rng().gen_range(0..101) / 10) as f64;
    let who_is_balder = thread_rng().gen_bool(0.5);

    if args.len() != 2 {
        msg.reply(
            &ctx.http,
            "Please send two people to compare their baldness, e.g. `^balder @GamerPaul @UnorigionalLeon`",
        )
        .await?;
        return Ok(());
    }
    let person1 = args.single::<String>()?;
    let person2 = args.single::<String>()?;
    let response = if who_is_balder {
        format!("{} is **{}x** balder than {}.", person1, hairlevel, person2)
    } else {
        format!("{} is **{}x** balder than {}.", person2, hairlevel, person1)
    };
    msg.reply(&ctx.http, &response).await?;
    Ok(())
}
