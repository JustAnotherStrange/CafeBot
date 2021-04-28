// idle increasing of user's money based on their purchased amount. starts at 0.
use crate::database::database::gen_connection;
use rusqlite::{params, Error};
use tokio_cron_scheduler::{Job, JobScheduler};
struct Interest {
    id: u64,
    incr_amount: u64,
}

pub async fn start_interest() {
    let mut sched = JobScheduler::new();
    sched
        .add(
            Job::new("0 0 * * * *", |_, _| {
                interest().unwrap();
            })
            .unwrap(),
        )
        .unwrap();
    sched.start().await.unwrap();
}

fn interest() -> Result<(), Error> {
    let conn = gen_connection();
    let mut stmt_incr = conn.prepare("update users set money = money + ?1 where id = ?2")?;
    let mut stmt = conn.prepare("select * from users")?;

    let rows = stmt.query_map([], |row| {
        Ok(Interest {
            id: row.get(0).unwrap(),
            incr_amount: row.get(3).unwrap(),
        })
    })?;
    let mut intr_vec = Vec::new();
    for intr in rows {
        intr_vec.push(intr.unwrap());
    }
    for intr in intr_vec.iter() {
        stmt_incr.execute(params![intr.incr_amount, intr.id])?;
    }
    Ok(())
}
