use std::env;
use std::fs;
use std::io::prelude::*;
use std::fs::OpenOptions;

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
    model::{channel::Message, gateway::Ready}, // misc::Mentionable},
    // prelude::*,
    utils::{MessageBuilder, content_safe, ContentSafeOptions},
};
struct Handler;

#[group]
#[commands(say, ping, count)]
struct General;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
    async fn message(&self, ctx: Context, msg: Message) {
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
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("log")
        .expect("failed to open log file");
    let content_to_append = MessageBuilder::new()
        .push(content)
        .push(" written by ")
        .push(&msg.author.name)
        .push(" using the say command in the channel ")
        .push(msg.channel_id) // time
        .push("\n")
        .build();
    file.write_all(content_to_append.as_bytes()).expect("failed to write content to log file");
    Ok(())
}

#[command]
#[only_in(guilds)]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "pong").await?;
    Ok(())
}

#[command]
#[only_in(guilds)]
async fn count(ctx: &Context, msg: &Message) -> CommandResult {
    let mut file = fs::read_to_string("./count").expect("Unable to read file.");
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
