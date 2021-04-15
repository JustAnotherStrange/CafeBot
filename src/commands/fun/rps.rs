use rand::{thread_rng, Rng};
use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};

#[command]
#[aliases("rps")]
async fn rockpaperscissors(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let turn: u32 = thread_rng().gen_range(1..4);
    // 1 - rock, 2 - paper, 3 - scissors
    let translate = match turn {
        1 => "rock",
        2 => "paper",
        3 => "scissors",
        _ => panic!("uhhh translation failed"),
    };
    let winresponse = format!("I picked {} - you win!", translate);
    let loseresponse = format!("I picked {} - you lose!", translate);
    let tieresponse = format!("I picked {} - tie!", translate);
    match args.rest() {
        // user picks rock
        "rock" => match turn {
            3 => msg.reply(&ctx.http, winresponse).await?,
            2 => msg.reply(&ctx.http, loseresponse).await?,
            _ => msg.reply(&ctx.http, tieresponse).await?,
        },
        "paper" => match turn {
            1 => msg.reply(&ctx.http, winresponse).await?,
            3 => msg.reply(&ctx.http, loseresponse).await?,
            _ => msg.reply(&ctx.http, tieresponse).await?,
        },
        "scissors" => match turn {
            2 => msg.reply(&ctx.http, winresponse).await?,
            1 => msg.reply(&ctx.http, loseresponse).await?,
            _ => msg.reply(&ctx.http, tieresponse).await?,
        },
        _ => {
            msg.reply(&ctx.http, "Please enter rock, paper, or scissors")
                .await?
        }
    };
    Ok(())
}
