use crate::is_admin;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};
use std::{fs, fs::OpenOptions, io::Write};

#[command]
#[only_in(guilds)]
async fn custom(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if !(std::path::Path::new("customs").exists()) {
        fs::create_dir("customs")?; // if folder "customs" doesn't exist, create it.
    };
    // for custom commands different in each server
    let guildid = msg.guild_id.unwrap().as_u64().clone();
    let guildid_path = format!("customs/{}", guildid);
    if !(std::path::Path::new(&guildid_path).exists()) {
        fs::create_dir(&guildid_path)?;
    }
    let command_name = match args.single::<String>() {
        Ok(x) => x,
        Err(_) => {
            msg.reply(
                &ctx.http,
                "Please enter command in this format: ^custom [command] [output]",
            )
            .await?;
            return Ok(());
        }
    };
    let command_output = args.rest().to_string();
    let filename = format!("{}/{}", guildid_path, command_name);
    if !(std::path::Path::new(&filename).exists()) {
        fs::File::create(&filename)?;
    } else {
        let to_send = format!("The custom command *{}* already exists.", command_name);
        msg.reply(&ctx.http, &to_send).await?;
        return Ok(());
    }
    // only gets here if the command does not already exist
    let mut command_file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(&filename)
        .unwrap();
    match command_file.write_all(command_output.as_bytes()) {
        Ok(x) => x,
        Err(e) => println!("{:?}", e),
    };
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
async fn run(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let to_run = match args.single::<String>() {
        Ok(x) => x,
        Err(_) => {
            msg.reply(
                &ctx.http,
                "Please use this format: ^run [custom command name]",
            )
            .await?;
            return Ok(());
        }
    };
    // list commands
    let guildid = msg.guild_id.unwrap().as_u64().clone();
    if to_run == "list" {
        // make a string with all the things separated by \n
        let guild_path = format!("customs/{}", guildid);
        let dir = fs::read_dir(&guild_path).unwrap();
        let mut commands = String::new();
        let mut temp = String::new();
        let mut index = 1;
        for path in dir {
            temp.clear();
            temp = format!(
                "{}: **{:?}**\n",
                index,
                path.unwrap().path().file_name().unwrap()
            );
            commands.push_str(&temp);
            index += 1;
        }
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
    let command_path = format!("customs/{}/{}", guildid, to_run);
    let command_output = match fs::read_to_string(&command_path) {
        Ok(x) => x,
        Err(_) => {
            msg.reply(
                &ctx.http,
                "That command doesn't exist yet. Create it with ^custom.",
            )
            .await?;
            return Ok(());
        }
    };
    // check if second arg exists, and if it does, check if its "delete" or something else, in that case then say what you can do.
    let second_args = match args.single::<String>() {
        Ok(x) => x,
        Err(_) => {
            // this is where most uses of the command will get. no second argument, not ^r list, prints the command output.
            msg.reply(&ctx.http, &command_output).await?;
            return Ok(());
        }
    };
    return if second_args == "delete" {
        if is_admin(ctx, msg).await {
            fs::remove_file(command_path).unwrap();
            let to_send = format!(
                "Removed command *{}*, which had the output '{}'.",
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
    } else {
        msg.reply(
            &ctx.http,
            "Please use the syntax: ^run [command name], ^run [command name] delete, or ^run list.",
        )
        .await?;
        Ok(())
    };
}
