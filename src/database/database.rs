use rusqlite::{params, Connection, Result};
use serenity::model::prelude::*;
pub struct ScratchOff {
    pub tier1: u32,
    pub tier2: u32,
    pub tier3: u32,
}

pub fn db_init() -> Result<()> {
    let conn = gen_connection();
    // creates a table if it doesn't exist called "users"
    // it has two columns: id (discord id) and money (how much money they have)
    // the UNIQUE on the id column is so u can test to see if it already exists or not (see create_user function)
    conn.execute(
        "create table if not exists users(
    id int not null unique, money int not null, tickets int not null, incr_amount not null,
    so_tier1 int not null, so_tier2 int not null, so_tier3 int not null)",
        [],
    )?;
    // customs table
    conn.execute(
        "create table if not exists customs(
    guild_id int not null, name text not null, output text)",
        [],
    )?;
    // daily table
    conn.execute(
        "create table if not exists daily(
    id int not null unique, date text not null, streak int not null)",
        [],
    )?;
    // count
    conn.execute(
        "create table if not exists count(
    guild_id int not null, count int not null)",
        [],
    )?;
    conn.execute(
        "create table if not exists pool(
        guild_id int not null unique, amount int not null)",
        [],
    )?;
    Ok(())
}
pub fn gen_connection() -> Connection {
    Connection::open("data.db").expect("failed to open database")
}
pub fn create_user_if_not_exist(user: &User, conn: &Connection) -> Result<()> {
    // insert if not already exists
    conn.execute(
        "insert or ignore into users values (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![user.id.as_u64(), 10, 0, 2, 0, 0, 0],
    )?;
    Ok(())
}
pub fn money_increment(user: &User, guild_id: u64, amount: i32) -> Result<()> {
    let conn = gen_connection();
    create_user_if_not_exist(&user, &conn)?;
    if amount.is_negative() {
        pool_increment(guild_id, amount / -4, &conn)?;
    }
    conn.execute(
        "update users set money = money + ?1 where id = ?2",
        params![amount, user.id.as_u64()],
    )?;
    Ok(())
}

fn pool_increment(guild_id: u64, amount: i32, conn: &Connection) -> Result<()> {
    // create guild row if not exist
    conn.execute(
        "insert or ignore into pool values (?1, 0)",
        params![guild_id],
    )?;
    // increment
    conn.execute(
        "update pool set amount = amount + ?1 where guild_id = ?2",
        params![amount, guild_id],
    )?;
    Ok(())
}
pub fn get_pool(guild_id: u64) -> Result<u64> {
    let conn = gen_connection();
    conn.execute(
        "insert or ignore into pool values (?1, 0)",
        params![guild_id],
    )?;
    let money = conn.query_row(
        "select amount from pool where guild_id = ?1",
        params![guild_id],
        |row| Ok(row.get(0)?),
    );
    return money;
}
pub fn get_money(user: &User) -> Result<i32> {
    let conn = gen_connection();
    create_user_if_not_exist(&user, &conn)?;
    // let mut stmt = conn.prepare("select money from users where id = ?1")?;
    let money = conn.query_row(
        "select money from users where id = ?1",
        params![user.id.as_u64()],
        |row| Ok(row.get(0)?),
    );
    return money;
}

pub fn get_incr_amount(user: &User, conn: &Connection) -> i32 {
    // let mut stmt = conn.prepare("select money from users where id = ?1")?;
    let money = conn
        .query_row(
            "select incr_amount from users where id = ?1",
            params![user.id.as_u64()],
            |row| Ok(row.get(0).unwrap()),
        )
        .unwrap();
    return money;
}

pub fn get_so(user: &User, conn: &Connection) -> ScratchOff {
    let mut stmt = conn
        .prepare("select so_tier1, so_tier2, so_tier3 from users where id = ?1")
        .unwrap();
    return stmt
        .query_row(params![user.id.as_u64()], |row| {
            Ok(ScratchOff {
                tier1: row.get(0).unwrap(),
                tier2: row.get(1).unwrap(),
                tier3: row.get(2).unwrap(),
            })
        })
        .unwrap();
}
