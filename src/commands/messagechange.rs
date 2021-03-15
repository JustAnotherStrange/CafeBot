use owoify_rs::{Owoifiable, OwoifyLevel};
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
    utils::{content_safe, ContentSafeOptions},
};
use std::{
    fs::{File, OpenOptions},
    io::prelude::*,
    time::{SystemTime, UNIX_EPOCH},
};

async fn modify(ctx: &Context, msg: &Message, to_send: &str) -> CommandResult {
    let d = msg.delete(&ctx.http);
    // following match statement makes it so the new message will reply to the same message that
    // the one being deleted replied to, but only if it replied to a message.
    match &msg.referenced_message {
        Some(m) => {
            let m = m.reply(&ctx.http, &to_send);
            tokio::try_join!(d, m)?; // do both at the same time and continue once both return Ok(). It'll quit if one returns any Err()
        }
        None => {
            let m = msg.channel_id.say(&ctx.http, &to_send);
            tokio::try_join!(d, m)?;
        }
    }
    Ok(())
}

#[command]
// Say command: repeat back what the user types, and then delete the user's original message
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
    modify(ctx, msg, args.rest()).await?;
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

#[command]
#[aliases("s", "/s")]
// sarcasm command for tExT lIkE tHiS. By g_w1
async fn sarcasm(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mut sarcasted = sarcastify(&args.rest());
    sarcasted.insert_str(0, "@: ");
    sarcasted.insert_str(1, &msg.author.name);
    modify(ctx, msg, &sarcasted).await?;
    Ok(())
}

fn sarcastify(to_sarc: &str) -> String {
    let mut sarcasted = String::new();
    let mut cap: bool = true;
    for cur in to_sarc.chars() {
        // Make it be alternating caps/lowercase
        cap = !cap;
        // if it can't be uppercase, just use the same char
        let to_push = if cap {
            cur.to_uppercase().nth(0).unwrap_or(cur)
        } else {
            cur.to_lowercase().nth(0).unwrap_or(cur)
        };
        sarcasted.push(to_push);
    }
    sarcasted
}

#[command]
// owo command for text wike twis uwu
async fn owo(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mut response = args.rest().owoify(&OwoifyLevel::Uwu); // use owoify-rs crate
    response.insert_str(0, "@: ");
    response.insert_str(1, &msg.author.name);
    modify(ctx, msg, &response).await?;
    Ok(())
}
