use rusqlite::{params, Connection, OptionalExtension, Result};
use serenity::model::prelude::*;

pub struct ScratchOff {
    pub tier1: u32,
    pub tier2: u32,
    pub tier3: u32,
}

pub fn db_init() -> Result<()> {
    // initializing database with all needed tables
    let conn = gen_connection();

    // users and their stuff
    conn.execute(
        "create table if not exists users(
    id int not null unique, money int not null, tickets int not null, incr_amount not null,
    so_tier1 int not null, so_tier2 int not null, so_tier3 int not null)",
        [],
    )?;

    // custom commands
    conn.execute(
        "create table if not exists customs(
    guild_id int not null, name text not null, output text)",
        [],
    )?;

    // daily streaks
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

    // pool (should be moved into stats, but i think i'll just leave it like this)
    conn.execute(
        "create table if not exists pool(
        guild_id int not null unique, amount int not null)",
        [],
    )?;

    // stats
    conn.execute(
        "create table if not exists stats(
        blackjacks int not null, coin_flips int not null)",
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
        lost_increment(guild_id, amount * -1, &conn)?;
    }
    conn.execute(
        "update users set money = money + ?1 where id = ?2",
        params![amount, user.id.as_u64()],
    )?;
    Ok(())
}

fn lost_increment(guild_id: u64, amount: i32, conn: &Connection) -> Result<()> {
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

pub fn get_lost(guild_id: u64) -> Result<u64> {
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
    return conn
        .query_row(
            "select so_tier1, so_tier2, so_tier3 from users where id = ?1",
            params![user.id.as_u64()],
            |row| {
                Ok(ScratchOff {
                    tier1: row.get(0).unwrap(),
                    tier2: row.get(1).unwrap(),
                    tier3: row.get(2).unwrap(),
                })
            },
        )
        .unwrap();
}

pub fn get_daily_streak(user: &User) -> Option<u32> {
    let conn = gen_connection();
    return conn
        .query_row(
            "select * from daily where id = ?1",
            params![user.id.as_u64()],
            |row| Ok(row.get(2)?),
        )
        .optional()
        .unwrap();
}
