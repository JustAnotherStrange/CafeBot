// run daily to try to keep up a streak.
use crate::database::database::money_increment;
use chrono::{prelude::*, Duration};
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::prelude::*,
    prelude::*,
};
use std::{
    fs,
    fs::OpenOptions,
    io::{prelude::*, BufRead, BufReader},
};

#[command]
async fn daily(ctx: &Context, msg: &Message) -> CommandResult {
    if !(std::path::Path::new("daily").exists()) {
        fs::create_dir("daily").unwrap(); // if folder "daily" doesn't exist, create it.
    }
    // create unique file for each user based on User ID in the "daily" directory
    let filename = format!("daily/{}", msg.author);
    let mut new = false;
    if !(std::path::Path::new(&filename).exists()) {
        fs::File::create(&filename)?; // if a file with the filename of the ID of the author doesn't exist, create it.
        new = true; // set this for an if statement later (fixes a bug)
    }
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
        .open(&filename)
        .expect("failed to open daily file");
    let mut date_string = format!("{}", Local::now());
    date_string = format!("{}", &date_string[0..10]); // take only the first 10 characters of local time, which contain just the date
    let mut yesterday_string = format!("{}", Local::now() - Duration::hours(24));
    yesterday_string = format!("{}", &yesterday_string[0..10]); // calculate yesterday's date similarly
    let line_fn = get_content_of_last_line(&filename); // this function returns a tuple. String for content of last line, and usize for total amount of lines.
    let content_of_last_line = line_fn.0;
    let amount_of_lines = line_fn.1;
    if new {
        // if the file has just been created, allow the Day 1 (previously it would say "last line != yesterdays date, so fail"
        let content_to_write = format!("{}\n", date_string); // add newline to date_string
        file.write_all(content_to_write.as_bytes())
            .expect("failed to write content to daily file");
        money_increment(&msg.author, 10).unwrap();
        let response = format!("Daily complete! This is day 0. You got **10** monies.");
        msg.reply(&ctx.http, response).await?;
    } else {
        if content_of_last_line != date_string {
            // if previous is not today
            if content_of_last_line == yesterday_string {
                // if previous was yesterday
                let content_to_write = format!("{}\n", date_string);
                file.write_all(content_to_write.as_bytes())
                    .expect("failed to write content to daily file");
                let days = amount_of_lines + 1;
                let to_increment: i32;
                if days <= 50 {
                    to_increment = days as i32 * 10;
                } else {
                    to_increment = 500;
                };
                money_increment(&msg.author, to_increment)?;
                let response = format!(
                    "Daily complete! This is day {:?}. You got **{}** monies.",
                    days, to_increment
                );
                msg.reply(&ctx.http, &response).await?;
            } else {
                // if previous not yesterday, lose streak
                let response = format!(
                    "Streak lost! Your streak was {}. Run ^daily again to start again.",
                    amount_of_lines
                );
                msg.reply(&ctx.http, &response).await?;
                fs::remove_file(&filename).unwrap();
            }
        } else {
            // if previous was today
            let response = format!(
                "Sorry, you have already done your daily for today. Your current streak is {}.",
                amount_of_lines
            );
            msg.reply(&ctx.http, &response).await?;
        }
    }
    Ok(())
}

// returns the number of the last line and its content.
fn get_content_of_last_line(filename: &str) -> (String, usize) {
    let file = fs::File::open(&filename).expect("failed to open daily file");
    let reader = BufReader::new(file);
    let mut content_of_last_line = String::new();
    let mut amount_of_lines = 0;
    for (i, line) in reader.lines().enumerate() {
        // read line by line using BufReader
        amount_of_lines = i;
        content_of_last_line = line.unwrap();
    }
    amount_of_lines += 1; // human readable lines (starts at 1, not 0)
    return (content_of_last_line, amount_of_lines);
}
