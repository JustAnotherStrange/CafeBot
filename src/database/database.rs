use rusqlite::{params, Connection, Result};
use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::{channel::Message, prelude::*},
};
use std::fs;

pub async fn db_init() -> Result<()> {
    let conn = gen_connection();
    // creates a table if it doesn't exist called "users"
    // it has two columns: id (discord id) and money (how much money they have)
    // the UNIQUE on the id column is so u can test to see if it already exists or not (see create_user function)
    conn.execute(
        "create table if not exists users(
    id int not null unique, money int not null)",
        [],
    )
    .expect("database oof :sob:");
    Ok(())
}
pub fn gen_connection() -> Connection {
    Connection::open("users.db").expect("failed to open database")
}
pub fn create_user_if_not_exist(msg: &Message) -> Result<()> {
    let conn = gen_connection();
    // insert if not already exists
    conn.execute(
        "insert or ignore into users values (?1, ?2)",
        params![msg.author.id.as_u64(), 0],
    )?;
    Ok(())
}
pub fn money_increment(msg: &Message, amount: i32) -> Result<()> {
    let conn = gen_connection();
    conn.execute(
        "update users set money = money + ?1 where id = ?2",
        params![amount, msg.author.id.as_u64()],
    )?;
    Ok(())
}
pub fn get_money(msg: &Message) -> Result<i32> {
    let conn = gen_connection();
    // let mut stmt = conn.prepare("select money from users where id = ?1")?;
    let money = conn.query_row(
        "select money from users where id = ?1",
        params![msg.author.id.as_u64()],
        |row| Ok(row.get(0)?),
    );
    return money;
}
// async fn is_author_in_db(msg: &Message, conn: &Connection) -> Result<bool> {
//     let mut stmt = conn.prepare("select id from users where id=?1")?;
//     return stmt.exists(params![msg.author.id.as_u64()]);
// }
