// zote command for the precepts of zote from hollow knight.
// Uses a file in the crate's root directory "./zote" should have been pulled as apart of git clone.
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use rand::{thread_rng, Rng};
use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    utils::MessageBuilder,
};

#[command]
async fn zote(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let args_string = args.rest();
    let zote_line = if args_string == "random" {
        thread_rng().gen_range(0..58)
    } else {
        // if the argument can't be parsed into usize, then set the line num to 100,
        // which will trigger the "please enter a number" message.
        args_string.parse().unwrap_or(100)
    };
    // // ^zote all was removed due to being a recipe for spamming.
    // if args_string == "all" {
    //     // print all precepts
    //     let filename = "zote";
    //     let file = File::open(filename).expect("failed to open file");
    //     let reader = BufReader::new(file);
    //     for line in reader.lines() {
    //         let line = line.unwrap();
    //         // Say the line along with a zote emoji from CyberCafe.
    //         let response = MessageBuilder::new()
    //             .push("<:zote:809592148805681193> ")
    //             .push(&line)
    //             .build();
    //         msg.channel_id.say(&ctx.http, &response).await?;
    //     }
    // } else if args_string == "bald" {
    if args_string == "bald" {
        msg.reply(&ctx.http, "<:zote:809592148805681193> Precept Bald: 'Never Be Bald'. A head without hair will make you weaker in battle. You must avoid this at all costs by growing hair.").await?;
    } else if zote_line > 57 {
        msg.reply(
            &ctx.http,
            "Please select a number from 1 to 57, or 'random', or 'bald'.",
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
