use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
    utils::MessageBuilder,
};
use std::{
    fs,
    fs::File,
    io::{BufRead, BufReader},
};

use rand::{thread_rng, Rng};
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
