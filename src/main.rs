#![allow(non_snake_case)] // because of CafeBot crate name
mod commands;

use serenity::{
    async_trait,
    client::{Client, Context, EventHandler},
    framework::{standard::macros::group, StandardFramework},
    model::{
        channel::Message,
        gateway::{Activity, Ready},
        user::OnlineStatus,
    },
};

use commands::{
    admin::*, bruh::*, count::*, daily::*, latency::*, game::*, hair::*, help::*, owo::*,
    rps::*, sarcasm::*, say::*, xkcd::*, zote::*, ping::*, latency::*
};

// https://github.com/serenity-rs/serenity/blob/53d5007a8d119158b5f0eea0a883b88de8861ae5/examples/e05_command_framework/src/main.rs#L34
// A container type is created for inserting into the Client's `data`, which
// allows for data to be accessible across all events and framework commands, or
// anywhere else that has a copy of the `data` Arc.

struct Handler;

#[group]
// List of commands
#[commands(
    latency,
    say,
    ping,
    count,
    hair,
    help,
    zote,
    sarcasm,
    bruh,
    status,
    slow_mode,
    admin_test,
    owo,
    daily,
    xkcd,
    rockpaperscissors,
    game
)]

struct General;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        // inform when connected
        println!("Connected as {}", ready.user.name);
        ctx.set_presence(
            Some(Activity::playing("vid eo g ame s")),
            OnlineStatus::Online,
        )
        .await; // set status to "Playing vid eo g ame s" - other Activity::* - listening, competing, streaming
    }
    async fn message(&self, ctx: Context, msg: Message) {
        // ----- subreddit detecting and linking by g_w1 -----
        if !(msg.content.to_lowercase().contains("://reddit.com")) {
            if let Some(l) = &msg.content.find("r/") {
                if *l == 0 || msg.content.chars().collect::<Vec<char>>()[l - 1].is_whitespace() {
                    let mut sub_reddit = String::new();
                    for (i, c) in msg.content.chars().into_iter().enumerate() {
                        if i < *l + 2 {
                            // + 2 because of r/
                            continue;
                        }
                        if c == ' ' {
                            break;
                        }
                        sub_reddit.push(c);
                    }
                    if let Err(e) = msg
                        .reply(&ctx.http, format!("<https://reddit.com/r/{}>", sub_reddit))
                        .await
                    {
                        println!("error: {}", e);
                    }
                }
            }
        }
        // ----- end subreddit detecting -----
    }
}

#[tokio::main]
async fn main() {
    // Take token from the env var DISCORD_TOKEN
    let token = std::env::var("DISCORD_TOKEN")
        .expect("Expected a token from the environment variable DISCORD_TOKEN");
    let framework = StandardFramework::new()
        .configure(|c| {
            c // configure command framework with the prefix "^" and allow whitespaces (e.g. `^ ping")
                .with_whitespace(true)
                .prefix("^")
        })
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
