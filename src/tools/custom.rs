// Make, list, delete, and run custom commands that are unique to each server
use crate::database::database::{db_init, gen_connection};
use crate::sarcastify;
use owoify_rs::{Owoifiable, OwoifyLevel};
use rusqlite::{params, OptionalExtension};
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};

#[command]
#[only_in(guilds)]
// create custom commands
async fn custom(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    db_init()?; // will create customs table if not exist
                // for custom commands different in each guild
    let guildid = msg.guild_id.unwrap().as_u64().clone();
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
    if does_command_exist(guildid, command_name.clone()) {
        msg.reply(&ctx.http, "That command already exists.").await?;
        return Ok(());
    }

    // only gets here if the command does not already exist
    let command_output = args.rest().to_string(); // the rest of the arguments, which does not include the first word (because that was taken out earlier)
    let conn = gen_connection();
    conn.execute(
        "insert or ignore into customs values (?1, ?2, ?3)",
        params![guildid, command_name, command_output],
    )?;

    // open the command's file

    // complete!
    let to_send = format!(
        "**DB Success:** custom command *{}* created with the output *{}*.",
        command_name, command_output
    );
    msg.reply(&ctx.http, &to_send).await?;
    Ok(())
}
fn does_command_exist(guildid: u64, command_name: String) -> bool {
    let conn = gen_connection();
    let mut statement = conn
        .prepare("select * from customs where guild_id = ?1 and name = ?2")
        .unwrap();
    let query: Option<String> = statement
        .query_row(params![guildid, command_name], |row| Ok(row.get(1)?))
        .optional()
        .unwrap();
    return match query {
        Some(_) => true,
        None => false,
    };
}

fn get_command_output(guildid: u64, command_name: String) -> Option<String> {
    let conn = gen_connection();
    let mut statement = conn
        .prepare("select * from customs where guild_id = ?1 and name = ?2")
        .unwrap();
    return statement
        .query_row(params![guildid, command_name], |row| Ok(row.get(2)?))
        .optional()
        .unwrap();
}
fn get_list_of_commands(guildid: u64) -> String {
    let conn = gen_connection();
    let mut statement = conn.prepare("select * from customs where guild_id = ?1").unwrap();
    let rows: Option<String> = statement
        .query_row(params![guildid], |row| Ok(row.get(1)?))
        .optional()
        .unwrap();
    let mut commands = String::new();
    let mut i = 1;
    for command in rows {
        let to_push = format!("{}: **{}**\n", i, command);
        commands.push_str(to_push.as_str());
        i += 1;
    }
    return commands;
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
        let commands = get_list_of_commands(guildid);
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
    let command_output = match get_command_output(guildid, to_run) {
        Some(x) => x,
        None => {
            msg.reply(&ctx.http, "That command doesn't exist yet. Create it with `^custom`.").await?;
            return Ok(());
        },
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
        Ok(())
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
