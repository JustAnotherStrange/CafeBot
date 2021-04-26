use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::{channel::Message, id::UserId, misc::Mentionable},
};

use crate::database::database::gen_connection;

#[derive(Debug)]
struct Leader {
    id: u64,
    amount: i64,
}

#[command]
async fn leaderboard(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // let mut stmt = conn.prepare("select money from users where id = ?1")?;
    let choice = match args.single::<String>() {
        Ok(x) => x,
        Err(_) => {
            // No arguments
            msg.reply(&ctx.http, "Please enter in this syntax: `^leaderboard [choice]`. The available leaderboards are `money` and `daily`.")
                .await?;
            return Ok(());
        }
    };
    let leader_vec = get_leaderboard(choice.clone()).await.unwrap();
    let mut response = String::new();
    let mut index = 1;
    for cur in leader_vec {
        let to_push = format!(
            "#{}: {}: **{}** {}\n",
            index,
            UserId::from(cur.id).mention(),
            cur.amount,
            match choice.clone().as_str() {
                "money" => "monies",
                "daily" => "days",
                _ => "errors",
            }
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

async fn get_leaderboard(choice: String) -> Result<Vec<Leader>, ()> {
    let conn = gen_connection();

    // match argument to what statement to prepare
    let num_column: usize;
    let mut stmt = match choice.as_str() {
        "money" => {
            num_column = 1;
            conn.prepare("select * from users order by money desc")
                .unwrap()
        }
        "daily" => {
            num_column = 2;
            conn.prepare("select * from daily order by streak desc")
                .unwrap()
        }
        _ => return Err(()),
    };

    // Iterate over the rows and push each one's `name` with nice formatting.
    let rows = stmt
        .query_map([], |row| {
            Ok(Leader {
                id: row.get(0).unwrap(),
                amount: row.get(num_column).unwrap(),
            })
        })
        .unwrap();
    let mut leader_vec = Vec::new();
    for leader in rows {
        leader_vec.push(leader.unwrap());
    }

    return Ok(leader_vec);
}
