use chrono::{Local, Utc};
use cron::Schedule;
use std::{str::FromStr, time::Duration};

pub async fn start_scheduler() {
    //0   0   0/8  *   *  *
    let expression = "1/50   *   *     *       *  *  *";
    let schedule = Schedule::from_str(expression).unwrap();

    loop {
        let mut upcoming = schedule.upcoming(Utc).take(1);
        actix_rt::time::sleep(Duration::from_millis(500)).await;
        let local = &Local::now();

        if let Some(datetime) = upcoming.next() {
            if datetime.timestamp() <= local.timestamp() {

                println!("{:?}", "test");
            }
        }
    }
}