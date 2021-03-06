// TODO:
#![allow(non_snake_case)] // because of CafeBot crate name
use std::{
    env, fs,
    fs::{File, OpenOptions},
    io::{prelude::*, BufRead, BufReader},
    time::{SystemTime, UNIX_EPOCH},
};

use chrono::{prelude::*, Duration, Utc};
use owoify_rs::{Owoifiable, OwoifyLevel};
use rand::{thread_rng, Rng};
use serde_json::Value;

use serenity::{
    async_trait,
    client::{Client, Context, EventHandler},
    framework::{
        standard::{
            macros::{command, group},
            Args, CommandResult,
        },
        StandardFramework,
    },
    http::AttachmentType,
    model::{
        channel::{Channel, Message},
        gateway::{Activity, Ready},
        permissions::Permissions,
        user::OnlineStatus,
    },
    // prelude::*,
    utils::{content_safe, ContentSafeOptions, MessageBuilder},
};

// https://github.com/serenity-rs/serenity/blob/53d5007a8d119158b5f0eea0a883b88de8861ae5/examples/e05_command_framework/src/main.rs#L34
// A container type is created for inserting into the Client's `data`, which
// allows for data to be accessible across all events and framework commands, or
// anywhere else that has a copy of the `data` Arc.

struct Handler;

#[group]
// List of commands
#[commands(
    say, ping, count, hair, help, zote, sarcasm, latency, bruh, status, slow_mode, admin_test, owo,
    daily, xkcd
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
                    if let Err(oof) = msg
                        .reply(&ctx.http, format!("<https://reddit.com/r/{}>", sub_reddit))
                        .await
                    {
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
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
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
        fs::File::create("log")?; // create log file if it doesn't already exist
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
    file.write_all(content_to_log.as_bytes())
        .expect("failed to write content to log file");
    Ok(())
}

#[inline]
async fn modify(ctx: &Context, msg: &Message, to_send: &str) -> CommandResult {
    let d = msg.delete(&ctx.http);
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
    let day = Duration::hours(24);
    let mut yesterday_string = format!("{}", Local::now() - day);
    yesterday_string = format!("{}", &yesterday_string[0..10]); // calculate yesterday's date similarly
    let line_thing = get_content_of_last_line(&filename); // this function returns a tuple. String for content of last line, and usize for total amount of lines.
    let content_of_last_line = line_thing.0;
    let amount_of_lines = line_thing.1;
    if new {
        // if the file has just been created, allow the Day 1 (previously it would say "last line != yesterdays date, so fail"
        let content_to_log = format!("{}\n", date_string); // add newline to date_string
        file.write_all(content_to_log.as_bytes())
            .expect("failed to write content to log file");
        let response = MessageBuilder::new()
            .push("Daily complete! This is day ")
            .push_bold_safe(amount_of_lines)
            .push(".")
            .build();
        msg.reply(&ctx.http, &response).await?;
    } else {
        if content_of_last_line != date_string {
            // if previous is not today
            if content_of_last_line == yesterday_string {
                // if previous was yesterday
                let content_to_log = format!("{}\n", date_string);
                file.write_all(content_to_log.as_bytes())
                    .expect("failed to write content to log file");
                let response = format!("Daily complete! This is day {:?}.", amount_of_lines);
                msg.reply(&ctx.http, &response).await?;
            } else {
                // if previous not yesterday, lose streak
                msg.reply(&ctx.http, "Streak lost! Run ^daily again to start fresh.")
                    .await?;
                fs::remove_file(&filename).unwrap();
            }
        } else {
            // if previous was today
            msg.reply(
                &ctx.http,
                "Sorry, you have already done your daily for today.",
            )
            .await?;
        }
    }
    Ok(())
}

#[inline]
fn get_content_of_last_line(filename: &String) -> (String, usize) {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
        .open(&filename)
        .expect("failed to open daily file.");
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

#[command]
// TODO:
async fn xkcd(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let num = args.single::<u32>().unwrap_or(456789); // take the first argument and try to convert to u32. if fail, set to 456789 (for later)
    // make https request with reqwest to find the number of most recent comic
    let resp = reqwest::get("https://xkcd.com/info.0.json")
        .await?
        .text()
        .await?;
    let json: Value = serde_json::from_str(&resp)?; // json deserialize
    let max_num: u32 = format!("{}", json["num"]).trim().parse().unwrap(); // format the max num into i32
    if num > max_num {
        // if the number is too high. this will also trigger when it becomes 456789 after failing to parse into u32
        if args.rest() == "" {
            // if no arguments, send latest comic
            print_xkcd(max_num, msg, ctx).await?;
        } else if args.single::<String>().unwrap() == "random" {
            // if argument is "random", send a random comic
            let rand_num = thread_rng().gen_range(0..max_num);
            print_xkcd(rand_num, msg, ctx).await?;
        } else {
            // finally, if the arguments were neither nothing nor random, this means that they
            // entered a number too large or less than zero (due to the nature of hte u32 type)
            let response = format!(
                "Please enter no arguments, 'random', or a number between 1 and {}.",
                max_num
            );
            msg.reply(&ctx.http, &response).await?;
        }
    } else {
        // if number in between 1 and max_num, send its corresponding comic.
        print_xkcd(num, msg, ctx).await?;
        return Ok(());
    }
    Ok(())
}

// send xkcd comics by passing a u32 for the comics number
async fn print_xkcd(num: u32, msg: &Message, ctx: &Context) -> CommandResult {
    let link = format!("https://xkcd.com/{}/info.0.json", num); // insert number into link for metadata request
    let comic = reqwest::get(link).await?.text().await?; // make https request
    let json: Value = serde_json::from_str(&comic)?; // json parse metadata
    // set vars from metadata and format titles, dates, etc
    let title = format!(
        "**xkcd {}: {}**",
        json["num"].to_string(),
        rjq(json["safe_title"].to_string())
    );
    let date = format!(
        "{}-{}-{}",
        rjq(json["month"].to_string()),
        rjq(json["day"].to_string()),
        rjq(json["year"].to_string())
    );
    let image_link = rjq(json["img"].to_string());
    let desc = rjq(json["alt"].to_string());
    // send message with cool embed stuff and image link as an attachment
    let _ = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.content(&title);
            m.embed(|e| {
                // e.title(&title);
                e.description(&date);
                e.image("attachment://&image_link");
                e.footer(|f| {
                    f.text(&desc);
                    f
                });
                e
            });
            m.add_file(AttachmentType::Image(&image_link));
            m
        })
        .await;
    Ok(())
}

#[inline]
// remove json quotes
fn rjq(s: String) -> String {
    let mut st = String::from(&s); // because mutable String passing weird
    st.truncate(st.len() - 1); // remove ending quote
    return st[1..].to_string(); // remove beginning quote
}

#[command]
// ping pong command (used mostly for checking if bot is online)
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(&ctx.http, "pong").await?;
    Ok(())
}

#[command]
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
    let file_int: i32 = file
        .parse()
        .expect("Failed to parse file string into integer");
    let to_write = file_int + 1;
    let to_write_string = to_write.to_string();
    let to_write_final = String::new() + to_write_string.as_str() + "\n";
    fs::write("./count", to_write_final).expect("Failed to write to file"); // write the new number to the file
    msg.reply(&ctx.http, &to_write).await?;
    Ok(())
}

#[command]
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
        msg.reply(
            &ctx.http,
            "Please select a number from 1 to 57, or 'random', 'all', or 'bald'.",
        )
        .await?; // because there are only 57 precepts
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
#[aliases("bald")]
// baldness calculator (actually just a random number generator).
// You can also specify who to test (e.g. ^bald @joe)
async fn hair(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let hairlevel = thread_rng().gen_range(0..101);
    let args_string = args.rest();
    let response = MessageBuilder::new()
        .push_bold_safe(if args_string == "" {
            &msg.author.name
        } else {
            args_string
        }) // use the arguments for the person to be tested
        .push(" has ")
        .push_bold_safe(&hairlevel)
        .push("% hair.")
        .build();
    msg.reply(&ctx.http, &response).await?;
    Ok(())
}

#[command]
async fn bruh(ctx: &Context, msg: &Message) -> CommandResult {
    let choice = thread_rng().gen_range(1..5);
    match choice {
        1 => msg.reply(&ctx.http, "<:CyberBruh:702876356287135864>"),
        2 => msg.reply(&ctx.http, "<:burh:721124449252016158>"),
        3 => msg.reply(&ctx.http, "<:certifiedbruhmoment:704060742034522213>"),
        4 => msg.reply(&ctx.http, "<:bruh100:679483886241185823>"),
        _ => unreachable!(),
    }
    .await?;
    Ok(())
}
#[command]
// works but prints it as: -PT0.313701128S (this probably means 313ms)
async fn latency(ctx: &Context, msg: &Message) -> CommandResult {
    let sub: chrono::Duration = Utc::now() - msg.timestamp;
    msg.reply(
        &ctx.http,
        format!("latency is {} milliseconds", sub.num_milliseconds()),
    )
    .await?;
    Ok(())
}

#[command]
async fn status(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if let Some(member) = &msg.member {
        for role in &member.roles {
            if role
                .to_role_cached(&ctx.cache)
                .await
                .map_or(false, |r| r.has_permission(Permissions::ADMINISTRATOR))
            {
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
async fn admin_test(ctx: &Context, msg: &Message) -> CommandResult {
    if let Some(member) = &msg.member {
        for role in &member.roles {
            if role
                .to_role_cached(&ctx.cache)
                .await
                .map_or(false, |r| r.has_permission(Permissions::ADMINISTRATOR))
            {
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
            if role
                .to_role_cached(&ctx.cache)
                .await
                .map_or(false, |r| r.has_permission(Permissions::ADMINISTRATOR))
            {
                // if you have reached here, you are admin. now do the command.
                let say_content = if let Ok(slow_mode_rate_seconds) = args.single::<u64>() {
                    if let Err(why) = msg
                        .channel_id
                        .edit(&ctx.http, |c| c.slow_mode_rate(slow_mode_rate_seconds))
                        .await
                    {
                        format!(
                            "Failed to set slow mode to `{}` seconds. because {}",
                            slow_mode_rate_seconds, why
                        )
                    } else {
                        format!(
                            "Successfully set slow mode rate to `{}` seconds.",
                            slow_mode_rate_seconds
                        )
                    }
                } else if let Some(Channel::Guild(channel)) =
                    msg.channel_id.to_channel_cached(&ctx.cache).await
                {
                    format!(
                        "Current slow mode rate is `{}` seconds.",
                        channel.slow_mode_rate.unwrap_or(0)
                    )
                } else {
                    "Failed to find channel in cache.".to_string()
                };
                msg.reply(&ctx.http, say_content).await?;
                return Ok(());
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
    match args.rest() {
        "" => {
            let response = MessageBuilder::new()
                .push_bold_safe("Welcome to CafeBot!\n \n")
                .push("Commands:\n")
                .push("^help [page] - show help pages. Specify no page for the general help or use one of the following categories: admin\n")
                .push("^ping - pong\n")
                .push("^say - repeat anything that comes after this command\n")
                .push("^count - count as high as you can\n")
                .push("^hair [^bald] - see how bald you are.\n")
                .push("^zote - find precepts of zote. ^zote [number] for a specific precept, ^zote random for a random one, and ^zote bald for our own precept.\n")
                .push("^bruh - get a random bruh emote\n")
                .push("^latency - see latency to bot host.\n")
                .push("^sarcasm [^s, ^/s] - modify your message to bE lIkE tHiS.\n")
                .push("^owo - modify your message to be owoified.\n")
                .push("^daily - run this daily to maintain a streak. forgetting one day will result in a reset.\n")
                .push("^xkcd - get xkcd comics. run with no arguments for the latest, 'random' for a random comic, or a number of the specific one you want.\n")
                .build();
            msg.reply(&ctx.http, &response).await?;
        }
        "admin" => {
            if let Some(member) = &msg.member {
                // only let admins ask for admin help
                for role in &member.roles {
                    if role
                        .to_role_cached(&ctx.cache)
                        .await
                        .map_or(false, |r| r.has_permission(Permissions::ADMINISTRATOR))
                    {
                        let response = MessageBuilder::new()
                            .push_bold_safe("CafeBot admin\n \n")
                            .push("^admin_test - test if you are an admin\n")
                            .push("^status [string] - change the bot's status (will display as 'Playing [entered status]')\n")
                            .push("^slow_mode [seconds] - set the slow mode in that channel to a certain amount of seconds. Set to 0 to turnoff slow mode.\n")
                            .build();
                        msg.reply(&ctx.http, &response).await?;
                        break;
                    }
                }
            } else {
                msg.reply(&ctx.http, "You can't access this help page")
                    .await?;
            }
        }
        _ => {
            msg.reply(&ctx.http, "Please enter either no category for general help or one of these categories: admin.").await?;
        }
    }
    Ok(())
}
