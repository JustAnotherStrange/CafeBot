// make a number go up
use crate::database::database::gen_connection;
use rusqlite::{params, Error, OptionalExtension};
use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

#[command]
#[only_in(guilds)]
// count command for increasing a counter every time it's ran.
async fn count(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = msg.guild_id.unwrap().as_u64().clone();
    let num = get_number_or_create(&guild_id)?;
    msg.reply(&ctx.http, &num.to_string()).await?;
    number_increment(&guild_id)?;
    Ok(())
}

fn get_number_or_create(guild_id: &u64) -> Result<u64, Error> {
    let conn = gen_connection();
    let money: Option<u64> = conn
        .query_row(
            "select count from count where guild_id = ?1",
            params![guild_id],
            |row| Ok(row.get(0)?),
        )
        .optional()?;
    return match money {
        Some(x) => Ok(x),
        None => {
            // create new guild
            conn.execute("insert into count values (?1, 0)", params![guild_id])?;
            Ok(0)
        }
    };
}

fn number_increment(guild_id: &u64) -> Result<(), Error> {
    let conn = gen_connection();
    conn.execute("update count set count = count + 1 where guild_id = ?1", params![guild_id])?;
    Ok(())
}
