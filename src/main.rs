// TODO:
// add more logging
#![allow(non_snake_case)]
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufRead, BufReader};
use std::fs::OpenOptions;
use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};


use serenity::{
    async_trait,
    client::{Client, Context, EventHandler},
    framework::{
        StandardFramework,
        standard::{
            Args, CommandResult,
            macros::{command, group},
        },
    },
    model::{channel::Message, gateway::Ready},
    // prelude::*,
    utils::{MessageBuilder, content_safe, ContentSafeOptions},
};
struct Handler;

#[group]
#[commands(say, ping, count, hair, help, zote, sarcasm)]
struct General;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
    async fn message(&self, ctx: Context, msg: Message) {
        // ----- subreddit detecting and linking ----- 
        let mut sub_reddit = String::new();
        if !(msg.content.to_lowercase().contains("://reddit.com")) {
            if let Some(l) = &msg.content.find("r/") {
                for (i,c) in msg.content.chars().into_iter().enumerate() {
                    if i < *l + 2 { // + 2 because of r/
                        continue;
                    }
                    if c == ' ' {
                        break;
                    }
                    sub_reddit.push(c);
                }
                if let Err(oof) = msg.channel_id.say(&ctx.http, format!("<https://reddit.com/r/{}>", sub_reddit)).await {
                    println!("oofed: {}", oof);
                }
            }
        }
        // ----- end subreddit detecting -----
        
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");
    let framework = StandardFramework::new()
        .configure(|c| c
                   .with_whitespace(true)
                   .prefix("^"))
        .group(&GENERAL_GROUP);
    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
#[command]
#[only_in(guilds)]
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
    let content = content_safe(&ctx.cache, &args.rest(), &settings).await;
    if content == "bruh" {
        msg.channel_id.say(&ctx.http, "you have unlocked the secret response").await?;
    } else {
        msg.channel_id.say(&ctx.http, &content).await?;
    }
    msg.delete(&ctx.http).await?;
    if !(std::path::Path::new("log").exists()) {
        let _file = fs::File::create("log")?;
    }
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("log")
        .expect("failed to open log file");
    let start = SystemTime::now();
    let unixtime = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let content_to_log = MessageBuilder::new()
        .push("at ")
        .push(unixtime.as_secs())
        .push(": ")
        .push(content)
        .push(" written by ")
        .push(&msg.author.name)
        .push(" using the say command in the channel ")
        .push(msg.channel_id) // time
        .push("\n")
        .build();
    file.write_all(content_to_log.as_bytes()).expect("failed to write content to log file");
    Ok(())
}
#[command]
#[only_in(guilds)]
async fn sarcasm(ctx: &Context, msg: &Message, a: Args) -> CommandResult {
    let mut sarcasted = sarcastify(&a.rest());
    sarcasted.insert_str(0, "@: ");
    sarcasted.insert_str(1, &msg.author.name);
    msg.channel_id.say(&ctx.http, &sarcasted).await?;
    msg.delete(&ctx.http).await?;
    Ok(())
}
fn gen_random_bool() -> bool {
    let mut rng = rand::thread_rng();
    rng.gen::<bool>()
}

fn sarcastify(s: &str) -> String {
    let mut st = String::new();
    for c in s.chars() {
        // if it cant be uppercase, just use the same char
        let ch = if gen_random_bool() { c.to_uppercase().nth(0).unwrap_or(c) } else { c.to_lowercase().nth(0).unwrap_or(c) };
        st.push(ch);
    }
    st
}

#[command]
#[only_in(guilds)]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "pong").await?;
    // Example Log code. I could move it to a function because its pretty chonky. We'll see.
    // let mut file = OpenOptions::new()
    //     .write(true)
    //     .append(true)
    //     .open("log")
    //     .expect("failed to open log file");
    // let start = SystemTime::now();
    // let unixtime = start
    //     .duration_since(UNIX_EPOCH)
    //     .expect("Time went backwards");
    // let content_to_log = MessageBuilder::new()
    //     .push("at ")
    //     .push(unixtime.as_secs())
    //     .push(": ")
    //     .push(&msg.author.name)
    //     .push(" used the ping command in ")
    //     .push(msg.channel_id) // time
    //     .push("\n")
    //     .build();
    // file.write_all(content_to_log.as_bytes()).expect("failed to write content to log file");
    Ok(())
}

#[command]
#[only_in(guilds)]
async fn count(ctx: &Context, msg: &Message) -> CommandResult {
    // check if file doesn't exist and then create it if it doesn't
    if !(std::path::Path::new("count").exists()) {
        let _file = fs::File::create("count")?;
    }
    let mut file = fs::read_to_string("./count").expect("Unable to read file.");
    if file == "" {
        let to_write_final = String::new() + "0" + "\n";
        fs::write("./count", to_write_final).expect("Failed to write to file");
    }
    let len = file.len();
    file.truncate(len - 1);
    let file_int: i32 = file.parse().expect("Failed to parse file string into integer");
    let to_write = file_int + 1;
    let to_write_string = to_write.to_string();
    let to_write_final = String::new() + to_write_string.as_str() + "\n";
    fs::write("./count", to_write_final).expect("Failed to write to file");
    msg.channel_id.say(&ctx.http, &to_write).await?;
    Ok(())
}

#[command]
#[only_in(guilds)]
async fn zote(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let args_string = args.rest();
    // let zote_test: i32 = args_string.parse() {
    let zote_line: usize;
    if args_string == "random" {
        zote_line = gen_random_zote();
    } else if args_string == "all" {
        zote_line = 101;
    } else {
        zote_line = args_string.parse().unwrap_or(100);
    }
    if zote_line == 101 {
        // print all
        let filename = "zote";
        let file = File::open(filename).expect("failed to open file");
        let reader = BufReader::new(file);
        for (_index, line) in reader.lines().enumerate() {
            let line = line.unwrap();
            let response = MessageBuilder::new()
                .push("<:zote:809592148805681193> ")
                .push(&line)
                .build();
            msg.channel_id.say(&ctx.http, &response).await?;
        }
    } else if zote_line > 57 {
        msg.channel_id.say(&ctx.http, "Please select a number less than or equal to 57 and greater than 0").await?;
    } else {
        // take that line of the zote file and print it.
        let filename = "zote";
        let file = File::open(filename).expect("failed to open file");
        let reader = BufReader::new(file);
        for (index, line) in reader.lines().enumerate() {
            let line = line.unwrap(); // Ignore errors.
            // Show the line and its number.
            if index + 1 == zote_line {
                let response = MessageBuilder::new()
                    .push("<:zote:809592148805681193> ")
                    .push(&line)
                    .build();
                msg.channel_id.say(&ctx.http, &response).await?;
                break;
            }
        }
    };
    Ok(())
}
fn gen_random_zote() -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..58)
}

#[command]
#[only_in(guilds)]
#[aliases("bald")]
async fn hair(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    // let settings = ContentSafeOptions::default();
    let hairlevel = gen_hairlevel();
    let content = args.rest();
    if content == "" {
        let response = MessageBuilder::new()
            .push_bold_safe(&msg.author.name)
            .push(" has ")
            .push_bold_safe(&hairlevel)
            .push("% hair.")
            .build();
        msg.channel_id.say(&ctx.http, &response).await?;
    } else {
        let response = MessageBuilder::new()
            .push_bold_safe(&content)
            .push(" has ")
            .push_bold_safe(&hairlevel)
            .push("% hair.")
            .build();
        msg.channel_id.say(&ctx.http, &response).await?;
    }
    Ok(())
}

fn gen_hairlevel() -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..101)
}

#[command]
#[only_in(guilds)]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    let response = MessageBuilder::new()
        .push_bold_safe("Welcome to CafeBot!\n \n")
        .push("Commands:\n")
        .push("^help - show help page\n")
        .push("^ping - pong\n")
        .push("^say - repeat anything that comes after this command\n")
        .push("^count - count as high as you can\n")
        .push("^hair - see how bald you are (also ^bald) \n")
        .push("^zote - find precepts of zote. ^zote [number] for a specific precept, and ^zote random for a random one.\n")
        .build();
    msg.channel_id.say(&ctx.http, &response).await?;
    Ok(())
}
