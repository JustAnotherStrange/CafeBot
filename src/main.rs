// TODO:
#![allow(non_snake_case)] // because of CafeBot name of crate
use std::{
    env, 
    fs, fs::{File, OpenOptions}, 
    io::{BufRead, BufReader, prelude::*},
    time::{SystemTime, UNIX_EPOCH},
    // collections::{HashMap, HashSet}, fmt::Write, sync::Arc
    sync::Arc
};

use rand::{thread_rng, Rng};

use serenity::{
    async_trait,
    client::{Client, Context, EventHandler, bridge::gateway::{ShardId, ShardManager}},
    framework::{
        StandardFramework,
        standard::{
            Args, CommandResult,
            macros::{command, group},
        },
    },
    // http::Http,
    model::{channel::{Message, Channel}, gateway::{Ready, Activity}, user::OnlineStatus, permissions::Permissions},
    prelude::*,
    utils::{MessageBuilder, content_safe, ContentSafeOptions},
};

// https://github.com/serenity-rs/serenity/blob/53d5007a8d119158b5f0eea0a883b88de8861ae5/examples/e05_command_framework/src/main.rs#L34
// A container type is created for inserting into the Client's `data`, which
// allows for data to be accessible across all events and framework commands, or
// anywhere else that has a copy of the `data` Arc.
struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct Handler;

#[group]
// List of commands 
#[commands(say, ping, count, hair, help, zote, sarcasm, latency, bruh, status, slow_mode, admin_test)]
struct General;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) { // inform when connected
        println!("Connected as {}", ready.user.name);
        let activity = Activity::playing("vid eo g ame s"); // other Activity::* - listening, competing, streaming
        ctx.set_presence(Some(activity), OnlineStatus::Online).await; // set status to "Playing vid eo g ame s"
    }
    async fn message(&self, ctx: Context, msg: Message) {
        // ----- subreddit detecting and linking by g_w1 ----- 
        if !(msg.content.to_lowercase().contains("://reddit.com")) {
            if let Some(l) = &msg.content.find("r/") {
                if *l == 0 || msg.content.chars().collect::<Vec<char>>()[l - 1].is_whitespace() {
                    let mut sub_reddit = String::new();
                    for (i,c) in msg.content.chars().into_iter().enumerate() {
                        if i < *l + 2 { // + 2 because of r/
                            continue;
                        }
                        if c == ' ' {
                            break;
                        }
                        sub_reddit.push(c);
                    }
                    if let Err(oof) = msg.reply(&ctx.http, format!("<https://reddit.com/r/{}>", sub_reddit)).await {
                        println!("oofed: {}", oof);
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
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");
    let framework = StandardFramework::new()
        .configure(|c| c // configure command framework with the prefix "^" and allow whitespaces (e.g. `^ ping")
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
    delete_and_send(ctx, msg,  args.rest()).await?;
    if !(std::path::Path::new("log").exists()) {
        fs::File::create("log")?; // create log file if it doesn't already exist
    }
    // logging ---- 
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
        .open("log")
        .expect("failed to open log file");
    let start = SystemTime::now();
    let unixtime = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let content_to_log = MessageBuilder::new()
        .push("at ")
        .push(unixtime.as_secs())
        .push(": ")
        .push(content)
        .push(" written by ")
        .push(&msg.author.name)
        .push(" using the say command in the channel ")
        .push(msg.channel_id)
        .push("\n")
        .build();
    file.write_all(content_to_log.as_bytes()).expect("failed to write content to log file"); 
    Ok(())
}

#[inline] // what does this do? 
async fn delete_and_send(ctx: &Context, msg: &Message, to_send: &str) -> CommandResult {
    let d = msg.delete(&ctx.http);
    let m = msg.channel_id.say(&ctx.http, &to_send);
    tokio::try_join!(d,m)?; // do both at the same time and continue once both return Ok(). It'll quit if one returns any Err()
    Ok(())
}
#[command]
#[only_in(guilds)]
#[aliases("s", "/s")]
// sarcasm command for tExT lIkE tHiS. By g_w1 
async fn sarcasm(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mut sarcasted = sarcastify(&args.rest());
    sarcasted.insert_str(0, "@: ");
    sarcasted.insert_str(1, &msg.author.name);
    delete_and_send(ctx, msg, &sarcasted).await?;
    Ok(())
}

fn sarcastify(s: &str) -> String {
    let mut st = String::new();
    let mut cap: bool = true;
    for c in s.chars() {
        // Make it be alternating caps/lowercase
        cap = !cap;
        // if it can't be uppercase, just use the same char
        let ch = if cap { c.to_uppercase().nth(0).unwrap_or(c) } else { c.to_lowercase().nth(0).unwrap_or(c) };
        st.push(ch);
    }
    st
}

#[command]
#[only_in(guilds)]
// ping pong command
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(&ctx.http, "pong").await?;
    Ok(())
}

#[command]
#[only_in(guilds)]
// count command for increasing a counter every time it's ran.
// uses a "./count" file in the crate's root directory.
async fn count(ctx: &Context, msg: &Message) -> CommandResult {
    if !(std::path::Path::new("count").exists()) {
        fs::File::create("count")?; // create the count file if it doesn't already exist
    }
    let mut file = fs::read_to_string("./count").expect("Unable to read file.");
    // write "0" to file if the file is empty
    if file == "" {
        let to_write_final = String::new() + "0" + "\n";
        fs::write("./count", to_write_final).expect("Failed to write to file");
    }
    // convert the string from reading the file into an i32 for performing math on it
    let len = file.len();
    file.truncate(len - 1);
    let file_int: i32 = file.parse().expect("Failed to parse file string into integer");
    let to_write = file_int + 1;
    let to_write_string = to_write.to_string();
    let to_write_final = String::new() + to_write_string.as_str() + "\n";
    fs::write("./count", to_write_final).expect("Failed to write to file"); // write the new number to the file
    msg.reply(&ctx.http, &to_write).await?;
    Ok(())
}

#[command]
#[only_in(guilds)]
// zote command for the precepts of zote from hollow knight.
// Uses a file in the crate's root directory "./zote" should have been pulled as apart of git clone.
async fn zote(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let args_string = args.rest();
    let zote_line = if args_string == "random" {
        thread_rng().gen_range(0..58)
    } else {
        // if the argument can't be parsed into usize, then set the line num to 100,
        // which will trigger the "please enter a number" message.
        args_string.parse().unwrap_or(100)
    };
    if args_string == "all" {
        // print all precepts
        let filename = "zote";
        let file = File::open(filename).expect("failed to open file");
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line.unwrap();
            // Say the line along with a zote emoji from CyberCafe.
            let response = MessageBuilder::new()
                .push("<:zote:809592148805681193> ")
                .push(&line)
                .build();
            msg.channel_id.say(&ctx.http, &response).await?;
        }
    } else if args_string == "bald" {
        msg.reply(&ctx.http, "<:zote:809592148805681193> Precept Bald: 'Never Be Bald'. A head without hair will make you weaker in battle. You must avoid this at all costs by growing hair.").await?;
    } else if zote_line > 57 {
        msg.reply(&ctx.http, "Please select a number from 1 to 57, or 'random', 'all', or 'bald'.").await?; // because there are only 57 precepts
    } else {
        // take that line of the zote file and print it.
        let filename = "zote";
        let file = File::open(filename).expect("failed to open file");
        let reader = BufReader::new(file);
        for (index, line) in reader.lines().enumerate() {
            if index + 1 == zote_line {
                let line = line.unwrap(); 
                // Say the line along with a zote emoji from CyberCafe.
                let response = MessageBuilder::new()
                    .push("<:zote:809592148805681193> ")
                    .push(&line)
                    .build();
                msg.reply(&ctx.http, &response).await?;
                break;
            }
        }
    }
    Ok(())
}

#[command]
#[only_in(guilds)]
#[aliases("bald")]
// baldness calculator (actually just a random number generator).
// You can also specify who to test (e.g. ^bald @joe)
async fn hair(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let hairlevel = thread_rng().gen_range(0..101);
    let args_string = args.rest();
    let response = MessageBuilder::new()
        .push_bold_safe(if args_string == "" { &msg.author.name } else { args_string }) // use the arguments for the person to be tested
        .push(" has ")
        .push_bold_safe(&hairlevel)
        .push("% hair.")
        .build();
    msg.reply(&ctx.http, &response).await?;
    Ok(())
}

#[command]
#[only_in(guilds)]
async fn bruh(ctx: &Context, msg: &Message) -> CommandResult {
    let choice = thread_rng().gen_range(1..5);
    match choice {
        1 => msg.reply(&ctx.http, "<:CyberBruh:702876356287135864>"),
        2 => msg.reply(&ctx.http, "<:burh:721124449252016158>"),
        3 => msg.reply(&ctx.http, "<:certifiedbruhmoment:704060742034522213>"),
        4 => msg.reply(&ctx.http, "<:bruh100:679483886241185823>"),
        _ => msg.reply(&ctx.http, "random number generation range broke."),
    }.await?;
    Ok(())
}
#[command]
#[only_in(guilds)]
// from https://github.com/serenity-rs/serenity/blob/53d5007a8d119158b5f0eea0a883b88de8861ae5/examples/e05_command_framework/src/main.rs#L437
async fn latency(ctx: &Context, msg: &Message) -> CommandResult {
    // The shard manager is an interface for mutating, stopping, restarting, and
    // retrieving information about shards.
    let data = ctx.data.read().await;

    let shard_manager = match data.get::<ShardManagerContainer>() {
        Some(v) => v,
        None => {
            msg.reply(ctx, "There was a problem getting the shard manager").await?;

            return Ok(());
        },
    };

    let manager = shard_manager.lock().await;
    let runners = manager.runners.lock().await;

    // Shards are backed by a "shard runner" responsible for processing events
    // over the shard, so we'll get the information about the shard runner for
    // the shard this command was sent over.
    let runner = match runners.get(&ShardId(ctx.shard_id)) {
        Some(runner) => runner,
        None => {
            msg.reply(ctx,  "No shard found").await?;

            return Ok(());
        },
    };

    msg.reply(ctx, &format!("The shard latency is {:?}", runner.latency)).await?;

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn status(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if let Some(member) = &msg.member {
        for role in &member.roles {
            if role.to_role_cached(&ctx.cache).await.map_or(false, |r| r.has_permission(Permissions::ADMINISTRATOR)) {
                let name = args.message();
                ctx.set_activity(Activity::playing(&name)).await;
                let response = MessageBuilder::new()
                    .push("Status has been set to ")
                    .push_bold_safe("Playing")
                    .push(" ")
                    .push_bold_safe(&name)
                    .build();
                msg.reply(&ctx.http, &response).await?;
                return Ok(());
            }
        }
    }
    msg.reply(&ctx.http, "You can't run that command.").await?;
    Ok(())
}

// https://github.com/serenity-rs/serenity/blob/dcc1ac4d0a12f24e998af3949e33ec352153a6af/examples/e05_command_framework/src/main.rs#L522
#[command]
#[only_in(guilds)]
async fn admin_test(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    if let Some(member) = &msg.member {
        for role in &member.roles {
            if role.to_role_cached(&ctx.cache).await.map_or(false, |r| r.has_permission(Permissions::ADMINISTRATOR)) {
                msg.reply(&ctx.http, "You are an admin.").await?;
                return Ok(());
            }
        }
    }
    msg.reply(&ctx.http, "You are not an admin.").await?;
    Ok(())
}
#[command]
#[only_in(guilds)]
// Only people with administrator permissions can run
// #[required_permissions(ADMINISTRATOR)]
// taken from example on serenity github
// https://github.com/serenity-rs/serenity/blob/dcc1ac4d0a12f24e998af3949e33ec352153a6af/examples/e05_command_framework/src/main.rs#L540
async fn slow_mode(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if let Some(member) = &msg.member {
        for role in &member.roles {
            if role.to_role_cached(&ctx.cache).await.map_or(false, |r| r.has_permission(Permissions::ADMINISTRATOR)) {
                // if you have reached here, you are admin. now do the command.
                let say_content = if let Ok(slow_mode_rate_seconds) = args.single::<u64>() {
                    if let Err(why) = msg.channel_id.edit(&ctx.http, |c| c.slow_mode_rate(slow_mode_rate_seconds)).await {
                        format!("Failed to set slow mode to `{}` seconds. because {}", slow_mode_rate_seconds, why)
                    } else {
                        format!("Successfully set slow mode rate to `{}` seconds.", slow_mode_rate_seconds)
                    }
                } else if let Some(Channel::Guild(channel)) = msg.channel_id.to_channel_cached(&ctx.cache).await {
                    format!("Current slow mode rate is `{}` seconds.", channel.slow_mode_rate.unwrap_or(0))
                } else {
                    "Failed to find channel in cache.".to_string()
                };
                msg.reply(&ctx.http, say_content).await?;
            }
        }
    }
    msg.reply(&ctx.http, "You can't run that command.").await?;
    Ok(())
}

#[command]
#[only_in(guilds)]
// Simple help command. 
// I tried to make it use embeds but it was a hassle and didn't work after a lot of debugging.
async fn help(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    // build the message
    let args_string = args.rest();
    match args_string {
        "" => { 
                let response = MessageBuilder::new()
                .push_bold_safe("Welcome to CafeBot!\n \n")
                .push("Commands:\n")
                .push("^help [page] - show help pages. Specify no page for the general help or use one of the following categories: admin\n")
                .push("^ping - pong\n")
                .push("^say - repeat anything that comes after this command\n")
                .push("^count - count as high as you can\n")
                .push("^hair - see how bald you are (also ^bald) \n")
                .push("^zote - find precepts of zote. ^zote [number] for a specific precept, ^zote random for a random one, and ^zote bald for our own precept.\n")
                .push("^bruh - get a random bruh emote")
                .push("^latency - see latency to bot host. currently broken.\n")
                .build();
            msg.reply(&ctx.http, &response).await?;
        }
        "admin" => {
            if let Some(member) = &msg.member {
            // only let admins ask for admin help
                for role in &member.roles {
                    if role.to_role_cached(&ctx.cache).await.map_or(false, |r| r.has_permission(Permissions::ADMINISTRATOR)) {
                        let response = MessageBuilder::new()
                            .push_bold_safe("CafeBot admin\n \n")
                            .push("^admin_test - test if you are an admin\n")
                            .push("^status [string] - change the bot's status (will display as 'Playing [entered status]')\n")
                            .push("^slow_mode [seconds] - set the slow mode in that channel to a certain amount of seconds. Set to 0 to turnoff slow mode.\n")
                            .build();
                        msg.reply(&ctx.http, &response).await?;
                    }
                }
            } else {
                msg.reply(&ctx.http, "You can't access this help page").await?;
            }
        }
        _ => { msg.reply(&ctx.http, "Please enter either no category for general help or one of these categories: admin.").await?;},
    }
    Ok(())
}
