// Say command: repeat back what the user types, and then delete the user's original message
use std::{
    fs::{File, OpenOptions},
    io::Write,
    time::{SystemTime, UNIX_EPOCH},
};

use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    utils::{content_safe, ContentSafeOptions},
};

use crate::commands::messagechange::modify;

#[command]
async fn say(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let settings = if let Some(guild_id) = msg.guild_id {
        ContentSafeOptions::default()
            .clean_channel(false)
            .display_as_member_from(guild_id)
    } else {
        ContentSafeOptions::default()
            .clean_channel(false)
            .clean_role(false)
    };
    let content = content_safe(&ctx.cache, &args.rest(), &settings).await; // this content safety returns @invalid-user for every user ping weirdly
    modify::modify(ctx, msg, args.rest()).await?;
    if !(std::path::Path::new("log").exists()) {
        File::create("log")?; // create log file if it doesn't already exist
    }
    // logging ----
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
        .open("log")
        .expect("failed to open log file");
    let unixtime = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let content_to_log = format!(
        "at {}: {} written by {} using the say command in the channel {}\n",
        unixtime.as_secs(),
        content,
        &msg.author.name,
        msg.channel_id
    );
    file.write_all(content_to_log.as_bytes())
        .expect("failed to write content to log file");
    Ok(())
}
