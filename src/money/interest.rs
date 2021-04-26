use crate::database::database::{get_money, money_increment, gen_connection};
use rand::{seq::SliceRandom, thread_rng};
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::*,
    Error,
};
use std::{thread::sleep, time, time::Duration};
use tokio_cron_scheduler::{JobScheduler, JobToRun, Job};

pub async fn start_interest() -> Result<()> {
    let mut sched = JobScheduler::new();
    sched.add(Job::new("* 1/1 * * * *", |uuid, l| {
        let conn = gen_connection();
        let mut stmt = conn.prepare("update users set money = money + 1").unwrap();
        stmt.execute([]).unwrap();
        println!("all monies have increased by 1");
    }).unwrap()).unwrap();
    let thing = sched.start().await;
    return thing;
}