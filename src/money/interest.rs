use crate::database::database::gen_connection;
use tokio_cron_scheduler::{JobScheduler, Job};

pub async fn start_interest() {
    let mut sched = JobScheduler::new();
    sched.add(Job::new("0 0 * * * *", |_, _| {
        let conn = gen_connection();
        let mut stmt = conn.prepare("update users set money = money + 2").unwrap();
        stmt.execute([]).unwrap();
    }).unwrap()).unwrap();
    sched.start().await.unwrap();
}