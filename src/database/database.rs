use rusqlite::{params, Connection, Result};
use serenity::model::prelude::*;

pub fn db_init() -> Result<()> {
    let conn = gen_connection();
    // creates a table if it doesn't exist called "users"
    // it has two columns: id (discord id) and money (how much money they have)
    // the UNIQUE on the id column is so u can test to see if it already exists or not (see create_user function)
    conn.execute(
        "create table if not exists users(
    id int not null unique, money int not null)",
        [],
    )?;
    // customs table
    conn.execute(
        "create table if not exists customs(
    guild_id int not null, name text not null, output text",
        [],
    )?;
    Ok(())
}
pub fn gen_connection() -> Connection {
    Connection::open("data.db").expect("failed to open database")
}
pub fn create_user_if_not_exist(user: &User) -> Result<()> {
    let conn = gen_connection();
    // insert if not already exists
    conn.execute(
        "insert or ignore into users values (?1, ?2)",
        params![user.id.as_u64(), 10],
    )?;
    Ok(())
}
pub fn money_increment(user: &User, amount: i32) -> Result<()> {
    let conn = gen_connection();
    create_user_if_not_exist(&user)?;
    conn.execute(
        "update users set money = money + ?1 where id = ?2",
        params![amount, user.id.as_u64()],
    )?;
    Ok(())
}
pub fn get_money(user: &User) -> Result<i32> {
    let conn = gen_connection();
    create_user_if_not_exist(&user)?;
    // let mut stmt = conn.prepare("select money from users where id = ?1")?;
    let money = conn.query_row(
        "select money from users where id = ?1",
        params![user.id.as_u64()],
        |row| Ok(row.get(0)?),
    );
    return money;
}
