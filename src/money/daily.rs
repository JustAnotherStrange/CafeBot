// run daily to try to keep up a streak.
use crate::database::database::{gen_connection, money_increment};
use chrono::{prelude::*, Duration};
use rusqlite::{params, OptionalExtension};
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::prelude::*,
    prelude::*,
};
pub struct Daily {
    pub id: u64,
    pub date: String,
    pub streak: u32,
}
#[command]
async fn daily(ctx: &Context, msg: &Message) -> CommandResult {
    let conn = gen_connection();
    // dates
    let mut date_string = format!("{}", Local::now());
    date_string = format!("{}", &date_string[0..10]); // take only the first 10 characters of local time, which contain just the date
    let mut yesterday_string = format!("{}", Local::now() - Duration::hours(24));
    yesterday_string = format!("{}", &yesterday_string[0..10]); // calculate yesterday's date similarly

    let user = match get_daily_user(&msg.author) {
        Some(x) => x,
        None => {
            // if the file has just been created, allow the Day 1 (previously it would say "last line != yesterdays date, so fail"
            conn.execute(
                "insert into daily values (?1, ?2, ?3)",
                params![msg.author.id.as_u64(), date_string, 0],
            )?;
            money_increment(&msg.author, msg.guild_id.unwrap().as_u64().clone(), 10).unwrap();
            let response = format!("Daily complete! This is day 0. You got **10** monies.");
            msg.reply(&ctx.http, response).await?;
            return Ok(());
        }
    };

    let user_id = user.id;
    let date_from_db = user.date;
    let streak_from_db = user.streak;

    if date_from_db != date_string {
        // if previous is not today
        if date_from_db == yesterday_string {
            // if previous was yesterday
            conn.execute(
                "update daily set date = ?1, streak = streak + 1 where id = ?2",
                params![date_string, user_id],
            )?;
            // money add
            let days = streak_from_db + 1;
            let to_increment: i32;
            if days <= 50 {
                to_increment = days as i32 * 10;
            } else {
                to_increment = 500;
            };
            money_increment(
                &msg.author,
                msg.guild_id.unwrap().as_u64().clone(),
                to_increment,
            )?;
            let response = format!(
                "Daily complete! This is day {:?}. You got **{}** monies.",
                days, to_increment
            );
            msg.reply(&ctx.http, &response).await?;
        } else {
            // if previous not yesterday, lose streak
            let response = format!(
                "Streak lost! Your streak was {}. Run ^daily again to start again.",
                streak_from_db
            );
            msg.reply(&ctx.http, &response).await?;
            conn.execute("delete from daily where id = ?1", params![user_id])?;
        }
    } else {
        // if previous was today
        let response = format!(
            "Sorry, you have already done your daily for today. Your current streak is {}.",
            streak_from_db
        );
        msg.reply(&ctx.http, &response).await?;
    }
    Ok(())
}

// Get user from db as Daily struct.
pub fn get_daily_user(user: &User) -> Option<Daily> {
    let conn = gen_connection();
    let mut stmt = conn.prepare("select * from daily where id = ?1").ok()?;
    return stmt
        .query_row(params![user.id.as_u64()], |row| {
            Ok(Daily {
                id: row.get(0)?,
                date: row.get(1)?,
                streak: row.get(2)?,
            })
        })
        .optional()
        .unwrap();
    // the .optional() makes it return an Option, which can be used to check if there is or is not a row with the specified params
}

pub fn get_daily_streak(user: &User) -> Option<u32> {
    let conn = gen_connection();
    let mut stmt = conn.prepare("select * from daily where id = ?1").ok()?;
    return stmt
        .query_row(params![user.id.as_u64()], |row| {
            Ok(
                row.get(2)?,
            )
        })
        .optional()
        .unwrap();
}
