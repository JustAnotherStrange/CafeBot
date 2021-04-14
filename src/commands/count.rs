use std::fs;

use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

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
