use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::{channel::Message, id::UserId, misc::Mentionable},
};

use crate::database::database::gen_connection;

#[derive(Debug)]
struct Leader {
    id: u64,
    money: i64,
}

#[command]
async fn leaderboard(ctx: &Context, msg: &Message) -> CommandResult {
    // let mut stmt = conn.prepare("select money from users where id = ?1")?;
    let leader_vec = get_leaderboard().await;
    let mut response = String::new();
    let mut index = 1;
    for cur in leader_vec {
        let to_push = format!(
            "#{}: {}: **{}** monies\n",
            index,
            UserId::from(cur.id).mention(),
            cur.money
        );
        index += 1;
        response.push_str(&*to_push);
    }
    msg.channel_id
        .send_message(&ctx, |m| {
            m.embed(|e| {
                e.title("Leaderboard:");
                e.description(&response);
                e
            })
        })
        .await?;
    Ok(())
}

async fn get_leaderboard() -> Vec<Leader> {
    let conn = gen_connection();
    // Iterate over the rows and push each one's `name` with nice formatting.
    let mut stmt = conn
        .prepare("select * from users order by money desc")
        .unwrap();
    let rows = stmt
        .query_map([], |row| {
            Ok(Leader {
                id: row.get(0).unwrap(),
                money: row.get(1).unwrap(),
            })
        })
        .unwrap();
    let mut leader_vec = Vec::new();
    for leader in rows {
        leader_vec.push(leader.unwrap());
    }

    return leader_vec;
}
