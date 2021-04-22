use rusqlite::{params, Connection, Result};
use std::fs;
use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::{channel::Message, prelude::*},
};

pub async fn db_init() -> Result<()> {
    let conn = Connection::open("users.db").expect("failed to open database");
    Ok(())
}
async fn create_user(ctx: &Context, msg: &Message) -> CommandResult {
    Ok(())
}
async fn is_author_in_db(msg: &Message) -> bool {

    false
}