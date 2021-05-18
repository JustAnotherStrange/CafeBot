// Feed people's gambling addictions
use crate::{
    database::database::{gen_connection, get_money, money_increment},
    tools::stats::init_stats_if_necessary,
};
use rand::{thread_rng, Rng};
use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};

#[command]
#[aliases("cf")]
async fn coinflip(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    // increase game counter
    let conn = gen_connection();
    init_stats_if_necessary(&conn);
    conn.execute("update stats set coin_flips = coin_flips + 1", [])?;

    let bet: i32 = match args.rest().trim().parse() {
        Ok(x) => x,
        Err(_) => {
            msg.reply(&ctx.http, "Please enter an amount to bet as an argument.")
                .await?;
            return Ok(());
        }
    };
    if bet > get_money(&msg.author)? || bet < 0 {
        msg.reply(&ctx.http, "You can't bet more money than you have.")
            .await?;
        return Ok(());
    }
    if thread_rng().gen_bool(0.5) {
        money_increment(&msg.author, msg.guild_id.unwrap().as_u64().clone(), bet)?;
        let response = format!("You got heads! You got **{}** monies.", bet);
        msg.reply(&ctx.http, response).await?;
    } else {
        money_increment(&msg.author, msg.guild_id.unwrap().as_u64().clone(), -bet)?;
        let response = format!("You got tails! You lost **{}** monies.", bet);
        msg.reply(&ctx.http, response).await?;
    }
    Ok(())
}
