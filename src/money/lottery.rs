use crate::database::database::gen_connection;
use rusqlite::Error;
use rand::seq::SliceRandom;
use rand::thread_rng;

use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::prelude::*,
    prelude::*,
};
struct Candidate {
    id: u64,
    num_tickets: u64,
}

#[command]
async fn draw_lottery(ctx: &Context, msg: &Message) -> CommandResult {
    let conn = gen_connection();
    let mut stmt = conn.prepare("select * from users order by tickets")?;

    // Make a vec of candidate structs
    let rows = stmt.query_map([], |row| {
        Ok(Candidate {
            id: row.get(0)?,
            num_tickets: row.get(2)?,
        })
    })?;
    // make a vec with the user's id the amount of times that they have a ticket
    let mut lottery_vec: Vec<u64> = Vec::new();
    for candidate in rows {
        let candidate = candidate?;
        for _ in 0..candidate.num_tickets {
            lottery_vec.push(candidate.id);
        }
    }
    lottery_vec.shuffle(&mut thread_rng());
    let response = format!("Winner: {}", lottery_vec[0]);
    msg.reply(&ctx.http, response).await?;
    Ok(())
}
