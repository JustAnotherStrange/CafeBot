// Make, list, delete, and run custom commands that are unique to each server
use crate::{is_admin, sarcastify};
use owoify_rs::{Owoifiable, OwoifyLevel};
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};
use std::{fs, fs::OpenOptions, io::Write};

#[command]
#[only_in(guilds)]
// create custom commands
async fn custom(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if !(std::path::Path::new("customs").exists()) {
        fs::create_dir("customs")?; // if folder "customs" doesn't exist, create it.
    };
    // for custom commands different in each guild
    let guildid = msg.guild_id.unwrap().as_u64().clone();
    let guildid_path = format!("customs/{}", guildid); // unique folder for each guild
    if !(std::path::Path::new(&guildid_path).exists()) {
        fs::create_dir(&guildid_path)?; // create the guild's unique folder if it doesn't already exist
    }
    // Argument parsing
    let command_name = match args.single::<String>() {
        Ok(x) => x,
        Err(_) => {
            // gets here if there were no arguments at all
            msg.reply(
                &ctx.http,
                "Please enter command in this format: `^custom [command] [output]`.",
            )
            .await?;
            return Ok(());
        }
    };

    let command_output = args.rest().to_string(); // the rest of the arguments, which does not include the first word (because that was taken out earlier)
    let filename = format!("{}/{}", guildid_path, command_name); // file for each command, and the file's name is the name of the command
    if !(std::path::Path::new(&filename).exists()) {
        fs::File::create(&filename)?; // create the command's file if it doesn't exist already
    } else {
        // the command already exists
        let to_send = format!("The custom command *{}* already exists.", command_name);
        msg.reply(&ctx.http, &to_send).await?;
        return Ok(());
    }
    // only gets here if the command does not already exist
    // open the command's file
    let mut command_file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(&filename)
        .unwrap();
    command_file.write_all(command_output.as_bytes()).unwrap(); // write the desired contents to the command's file

    // complete!
    let to_send = format!(
        "**Success:** custom command *{}* created with the output *{}*.",
        command_name, command_output
    );
    msg.reply(&ctx.http, &to_send).await?;
    Ok(())
}

#[command]
#[only_in(guilds)]
#[aliases("r")]
// run, list, and delete the custom commands created with ^custom
async fn run(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let to_run = match args.single::<String>() {
        Ok(x) => x,
        Err(_) => {
            msg.reply(
                &ctx.http,
                "Please use this format: `^run [custom command name]`. You can also pipe with `^run [command_name] | [pipe_program]`.",
            )
            .await?;
            return Ok(());
        }
    };
    // list commands
    let guildid = msg.guild_id.unwrap().as_u64().clone();
    if to_run == "list" {
        // make a string with all the things separated by \n
        let guild_path = format!("customs/{}", guildid); // get the guild's unique foldr
        let dir = fs::read_dir(&guild_path).unwrap(); // read directory

        // format and print the contents of the guild's directory
        let mut commands = String::new();
        let mut temp = String::new();
        let mut index = 1;
        for path in dir {
            temp.clear(); // do I need this?
            temp = format!(
                "{}: **{:?}**\n",
                index,
                path.unwrap().path().file_name().unwrap()
            );
            commands.push_str(&temp);
            index += 1;
        }

        // send the formatted list in a nice embed
        msg.channel_id
            .send_message(&ctx.http, |m| {
                m.content("**Custom commands for this server:**");
                m.embed(|e| {
                    e.description(&commands);
                    e
                });
                m
            })
            .await?;
        return Ok(());
    }

    // read the command's file and output its contents
    let command_path = format!("customs/{}/{}", guildid, to_run);
    let command_output = match fs::read_to_string(&command_path) {
        Ok(x) => x,
        Err(_) => {
            // the file does not exist, so the command does not exist
            msg.reply(
                &ctx.http,
                "That command doesn't exist yet. Create it with `^custom`.",
            )
            .await?;
            return Ok(());
        }
    };

    // check if there is a second argument
    let second_args = match args.single::<String>() {
        Ok(x) => x,
        Err(_) => {
            // this is where most uses of the command will get. no second argument, not ^r list, prints the command output.
            msg.reply(&ctx.http, &command_output).await?;
            return Ok(());
        }
    };

    // if you get here, there is a second argument.
    return if second_args == "delete" {
        // delete a command by simply removing its file, but only if the user running the command is an admin.
        if is_admin(ctx, msg).await {
            fs::remove_file(command_path).unwrap();
            let to_send = format!(
                "Deleted command *{}*, which had the output *{}*.",
                to_run, command_output
            );
            msg.reply(&ctx.http, &to_send).await?;
            Ok(())
        } else {
            msg.reply(
                &ctx.http,
                "You aren't an admin, so you can't delete commands.",
            )
            .await?;
            Ok(())
        }
    } else if second_args == "|" {
        let mut modified_text = command_output;
        let mut next_next_args: String;
        loop {
            let next_args = match args.single::<String>() {
                Ok(x) => x,
                Err(_) => {
                    msg.reply(
                        &ctx.http,
                        "The syntax for piping is: `^run [command_name] | [pipe_program]`",
                    )
                    .await?;
                    return Ok(());
                }
            };

            // piping programs
            modified_text = match next_args.as_str() {
                "owo" => modified_text.owoify(&OwoifyLevel::Owo), // owoify using owoify-rs
                "uwu" => modified_text.owoify(&OwoifyLevel::Uwu), // owoify even more!!
                "uvu" => modified_text.owoify(&OwoifyLevel::Uvu), // owoify EVEN MORE?!?!?
                "sarcasm" => sarcastify(modified_text.as_str()), // use the same function that ^s uses
                _ => String::from(
                    "Please pipe into one of the following programs: owo, uwu, uvu, sarcasm.",
                ),
            };
            next_next_args = match args.single::<String>() {
                Ok(x) => x,
                Err(_) => break,
            };
            if next_next_args == "|" {
                continue;
            } else {
                break;
            }
        }
        msg.reply(&ctx.http, &modified_text).await?;
        Ok(())
    } else {
        // if there is second argument, but it is not "delete", then the user has done the wrong syntax.
        msg.reply(
            &ctx.http,
            "Please use the syntax: `^run [command name]`, `^run [command name] delete`, `^run list`, or pipe it using the symbol `|`.",
        )
        .await?;
        Ok(())
    };
}
