// TODO:
// instead of hardcoding my user id, use serenity's built in "owner" feature and make a new command
// group if necessary, where i specify owner ids in the framework declaration
#![allow(non_snake_case)] // because of CafeBot name of crate
use std::{
    env, 
    fs, fs::{File, OpenOptions}, 
    io::{BufRead, BufReader}, // prelude::*
    time::{SystemTime, UNIX_EPOCH},
    // collections::{HashMap, HashSet}, fmt::Write, sync::Arc
};

use rand::Rng;

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
    // http::Http,
    model::{channel::{Message, Channel}, gateway::{Ready, Activity}, user::OnlineStatus, permissions::Permissions},
    // prelude::*,
    utils::{MessageBuilder, content_safe, ContentSafeOptions},
};

struct Handler;

#[group]
// List of commands 
#[commands(say, ping, count, hair, help, zote, sarcasm, status, slow_mode, admin_test)]
struct General;

// owner's only commands
// #[group]
// #[owners_only]
// #[only_in(guilds)]
// // Summary only appears when listing multiple groups.
// #[summary = "Commands for server owners"]
// #[commands(slow_mode)]
// struct Owner;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) { // inform when connected
        println!("Connected as {}", ready.user.name);
        let activity = Activity::playing("vid eo g ame s"); // other Activity::* - listening, competing, streaming
        ctx.set_presence(Some(activity), OnlineStatus::Online).await; // set status to "Playing vid eo g ame s"
    }
    async fn message(&self, ctx: Context, msg: Message) {
        // ----- subreddit detecting and linking by g_w1 ----- 
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
    // Take token from the env var DISCORD_TOKEN
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");
    // We will fetch your bot's owners and id
    // let http = Http::new_with_token(&token);
    // let (owners, bot_id) = match http.get_current_application_info().await {
    //     Ok(info) => {
    //         let mut owners = HashSet::new();
    //         if let Some(team) = info.team {
    //             owners.insert(team.owner_user_id);
    //         } else {
    //             owners.insert(info.owner.id);
    //         }
    //         match http.get_current_user().await {
    //             Ok(bot_id) => (owners, bot_id.id),
    //             Err(why) => panic!("Could not access the bot id: {:?}", why),
    //         }
    //     },
    //     Err(why) => panic!("Could not access application info: {:?}", why),
    // };
    let framework = StandardFramework::new()
        .configure(|c| c // configure command framework with the prefix "^" and allow whitespaces (e.g. `^ ping")
                   .with_whitespace(true)
                   // .owners(owners)
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
    msg.channel_id.say(&ctx.http, &content).await?; 
    msg.delete(&ctx.http).await?;
    if !(std::path::Path::new("log").exists()) {
        let _file = fs::File::create("log")?; // create log file if it doesn't already exist
    }
    // logging ---- 
    let _file = OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
        .open("log")
        .expect("failed to open log file");
    let start = SystemTime::now();
    let unixtime = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let _content_to_log = MessageBuilder::new()
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
    // file.write_all(content_to_log.as_bytes()).expect("failed to write content to log file"); 
    // ---- FIX THIS. above is to write to log file. somehow this errors even though it didn't recently. any ideas anyone? 
    Ok(())
}
#[command]
#[only_in(guilds)]
// sarcasm command for tExT lIkE tHiS. By g_w1 
async fn sarcasm(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mut sarcasted = sarcastify(&args.rest());
    sarcasted.insert_str(0, "@: ");
    sarcasted.insert_str(1, &msg.author.name);
    msg.channel_id.say(&ctx.http, &sarcasted).await?;
    msg.delete(&ctx.http).await?;
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
    msg.channel_id.say(&ctx.http, "pong").await?;
    Ok(())
}

#[command]
#[only_in(guilds)]
// count command for increasing a counter every time it's ran.
// uses a "./count" file in the crate's root directory.
async fn count(ctx: &Context, msg: &Message) -> CommandResult {
    if !(std::path::Path::new("count").exists()) {
        let _file = fs::File::create("count")?; // create the count file if it doesn't already exist
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
    msg.channel_id.say(&ctx.http, &to_write).await?;
    Ok(())
}

#[command]
#[only_in(guilds)]
// zote command for the precepts of zote from hollow knight.
// Uses a file in the crate's root directory "./zote" should have been pulled as apart of git clone.
async fn zote(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let args_string = args.rest();
    let zote_line: usize; // usize because it's an indicator for what line to read from in the file
    if args_string == "random" {
        zote_line = gen_random_zote();
    } else if args_string == "all" {
        zote_line = 101; // 101 is an indicator to print all 57. probably a better way to do this, idk.
    } else {
        // if the argument can't be parsed into usize, then set the line num to 100,
        // which will trigger the "please enter a number" message.
        zote_line = args_string.parse().unwrap_or(100); 
    }
    if zote_line == 101 {
        // print all precepts
        let filename = "zote";
        let file = File::open(filename).expect("failed to open file");
        let reader = BufReader::new(file);
        for (_index, line) in reader.lines().enumerate() {
            let line = line.unwrap();
            // Say the line along with a zote emoji from CyberCafe.
            let response = MessageBuilder::new()
                .push("<:zote:809592148805681193> ")
                .push(&line)
                .build();
            msg.channel_id.say(&ctx.http, &response).await?;
        }
    } else if zote_line > 57 {
        msg.channel_id.say(&ctx.http, "Please select a number less than or equal to 57 and greater than 0").await?; // because there are only 57 precepts
    } else {
        // take that line of the zote file and print it.
        let filename = "zote";
        let file = File::open(filename).expect("failed to open file");
        let reader = BufReader::new(file);
        for (index, line) in reader.lines().enumerate() {
            let line = line.unwrap(); 
            if index + 1 == zote_line {
                // Say the line along with a zote emoji from CyberCafe.
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
fn gen_random_zote() -> usize { // again, usize because it's an indicator for a line number
    let mut rng = rand::thread_rng();
    rng.gen_range(0..58)
}

#[command]
#[only_in(guilds)]
#[aliases("bald")]
// baldness calculator (actually just a random number generator).
// You can also specify who to test (e.g. ^bald @joe)
async fn hair(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let hairlevel = gen_hairlevel();
    let args_string = args.rest();
    if args_string == "" { // if there is no argument
        // build a message where it says something like:
        // "@joe has 23% hair."
        let response = MessageBuilder::new()
            .push_bold_safe(&msg.author.name) // since there is no argument, just use the name of the author of the message for who is to be tested.
            .push(" has ")
            .push_bold_safe(&hairlevel)
            .push("% hair.")
            .build();
        msg.channel_id.say(&ctx.http, &response).await?;
    } else {
        // similarly build a message.
        let response = MessageBuilder::new()
            .push_bold_safe(&args_string) // use the arguments for the person to be tested
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
async fn status(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let name = args.message();
    ctx.set_activity(Activity::playing(&name)).await;
    let response = MessageBuilder::new()
        .push("Status has been set to ")
        .push_bold_safe("Playing")
        .push(" ")
        .push_bold_safe(&name)
        .build();
    msg.channel_id.say(&ctx.http, &response).await?;
    Ok(())
}

// https://github.com/serenity-rs/serenity/blob/dcc1ac4d0a12f24e998af3949e33ec352153a6af/examples/e05_command_framework/src/main.rs#L522
#[command]
#[only_in(guilds)]
async fn admin_test(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    if let Some(member) = &msg.member {
        for role in &member.roles {
            if role.to_role_cached(&ctx.cache).await.map_or(false, |r| r.has_permission(Permissions::ADMINISTRATOR)) {
                msg.channel_id.say(&ctx.http, "Yes, you are.").await?;
                return Ok(());
            }
        }
    }
    msg.channel_id.say(&ctx.http, "No, you are not.").await?;
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
                        println!("Error setting channel's slow mode rate: {:?}", why);
                        format!("Failed to set slow mode to `{}` seconds.", slow_mode_rate_seconds)
                    } else {
                        format!("Successfully set slow mode rate to `{}` seconds.", slow_mode_rate_seconds)
                    }
                } else if let Some(Channel::Guild(channel)) = msg.channel_id.to_channel_cached(&ctx.cache).await {
                    format!("Current slow mode rate is `{}` seconds.", channel.slow_mode_rate.unwrap_or(0))
                } else {
                    println!("cache fail");
                    "Failed to find channel in cache.".to_string()
                };
                msg.channel_id.say(&ctx.http, say_content).await?;
                return Ok(());
            }
        }
    }
    msg.channel_id.say(&ctx.http, "You can't run that command.").await?;
    Ok(())
}
#[command]
#[only_in(guilds)]
// Simple help command. 
// I tried to make it use embeds but it was a hassle and didn't work after a lot of debugging.
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    // build the message
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
