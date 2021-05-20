// rock paper scissors game
use crate::database::database::{get_money, money_increment_with_lost};
use rand::{thread_rng, Rng};
use rusqlite::Error;
use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};

#[command]
#[aliases("rps")]
async fn rockpaperscissors(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let turn: u32 = thread_rng().gen_range(1..4);
    // 1 - rock, 2 - paper, 3 - scissors
    let translate = match turn {
        1 => "rock",
        2 => "paper",
        3 => "scissors",
        _ => unreachable!(),
    };
    let user_move = match args.single::<String>() {
        Ok(x) => x,
        Err(_) => {
            msg.reply(
                &ctx.http,
                "Please enter in this syntax: `^rps [move] [bet]`",
            )
            .await?;
            return Ok(());
        }
    };
    let bet = match args.single::<i32>() {
        Ok(x) => x,
        Err(_) => {
            msg.reply(
                &ctx.http,
                "Please enter in this syntax: `^rps [move] [bet]`",
            )
            .await?;
            return Ok(());
        }
    };
    if bet > get_money(&msg.author)? || bet < 0 {
        msg.reply(&ctx.http, "You can't bet more money than you have.")
            .await?;
        return Ok(());
    }
    match user_move.as_str() {
        // user picks rock
        "rock" => match turn {
            3 => {
                msg.reply(&ctx.http, win(msg, translate, bet as i32)?)
                    .await?
            }
            2 => {
                msg.reply(&ctx.http, lose(msg, translate, bet as i32)?)
                    .await?
            }
            _ => msg.reply(&ctx.http, tie(translate)?).await?,
        },
        "paper" => match turn {
            1 => {
                msg.reply(&ctx.http, win(msg, translate, bet as i32)?)
                    .await?
            }
            3 => {
                msg.reply(&ctx.http, lose(msg, translate, bet as i32)?)
                    .await?
            }
            _ => msg.reply(&ctx.http, tie(translate)?).await?,
        },
        "scissors" => match turn {
            2 => {
                msg.reply(&ctx.http, win(msg, translate, bet as i32)?)
                    .await?
            }
            1 => {
                msg.reply(&ctx.http, lose(msg, translate, bet as i32)?)
                    .await?
            }
            _ => msg.reply(&ctx.http, tie(translate)?).await?,
        },
        _ => {
            msg.reply(
                &ctx.http,
                "Please enter in this syntax: `^rps [move] [bet]`",
            )
            .await?
        }
    };
    Ok(())
}
fn win(msg: &Message, comp_move: &str, to_increment: i32) -> Result<String, Error> {
    money_increment_with_lost(
        &msg.author,
        msg.guild_id.unwrap().as_u64().clone(),
        to_increment,
    )?;
    return Ok(format!(
        "I picked {} - you win! You got **{}** monies.",
        comp_move, to_increment
    ));
}
fn lose(msg: &Message, comp_move: &str, to_subtract: i32) -> Result<String, Error> {
    money_increment_with_lost(
        &msg.author,
        msg.guild_id.unwrap().as_u64().clone(),
        -to_subtract,
    )?;
    return Ok(format!(
        "I picked {} - you lose! You lost **{}** monies.",
        comp_move, to_subtract
    ));
}
fn tie(comp_move: &str) -> Result<String, Error> {
    return Ok(format!("I picked {} - tie!", comp_move));
}
