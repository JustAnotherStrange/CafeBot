use crate::{
    database::database::{gen_connection, get_lost},
    money::blackjack::edit_embed,
};
use rusqlite::Connection;
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::prelude::*,
    prelude::*,
};

struct Stats {
    blackjacks: u64,
    coin_flips: u64,
}

#[command]
#[only_in(guilds)]
async fn stats(ctx: &Context, msg: &Message) -> CommandResult {
    let conn = gen_connection();
    init_stats_if_necessary(&conn);

    // all the stuffs to be put into the response
    let name = msg.guild_id.unwrap().name(&ctx).await.unwrap(); // name of guild
    let guild_lost_amount = get_lost(*msg.guild_id.unwrap().as_u64()).unwrap(); // originally the "pool"
                                                                                // lost amount/"pool" for all servers combined
    let all_lost_amount: i64 = conn
        .query_row("select sum(amount) from pool", [], |row| Ok(row.get(0)?))
        .unwrap();
    // collective wealth of every user in the database
    let collective_wealth: i64 = conn
        .query_row("select sum(money) from users", [], |row| Ok(row.get(0)?))
        .unwrap();
    // game stats
    let stats = conn
        .query_row("select * from stats", [], |row| {
            Ok(Stats {
                blackjacks: row.get(0).unwrap(),
                coin_flips: row.get(1).unwrap(),
            })
        })
        .unwrap_or(Stats {
            blackjacks: 0,
            coin_flips: 0,
        });

    // formatting
    let title = format!("Stats for {}", name);
    let description = format!(
        "\
    **Money Stats**
    Total monies lost: **{}**
    Total monies lost on this server: **{}**
    Collective wealth of all users: **{}**

    **Gambling Stats (all since May 17th, 2021)**
    Games of blackjack played: **{}**
    Total coins flipped: **{}**",
        all_lost_amount, guild_lost_amount, collective_wealth, stats.blackjacks, stats.coin_flips
    );

    // use this reply/edit thing as a workaround for having an embed be a reply
    let mut message = msg.reply(&ctx.http, "**CafeBot Stats**").await?;
    edit_embed(&ctx, &mut message, title.as_str(), description.as_str()).await;
    Ok(())
}

pub fn init_stats_if_necessary(conn: &Connection) {
    // if nothing, make them zeroes
    let amount_of_rows: i32 = conn
        .query_row("select count(*) from stats", [], |row| Ok(row.get(0)?))
        .unwrap(); // get amount of rows
                   // if there are zero rows, then create the row that has the information in it
    if amount_of_rows == 0 {
        conn.execute("insert into stats values (0, 0)", []).unwrap();
    }
}
