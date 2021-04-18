use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};
use std::{fs, fs::OpenOptions, io::Write};

#[command]
#[only_in(guilds)]
#[aliases("create")]
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
    let guildid = msg.guild_id.unwrap().as_u64().clone();
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
    msg.reply(&ctx.http, &command_output).await?;
    Ok(())
}
