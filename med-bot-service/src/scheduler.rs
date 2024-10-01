use chrono::{Local, Utc};
use cron::Schedule;
use std::{str::FromStr, time::Duration};
use actix_web::web::Data;
use crate::AppState;
use crate::services::med_service::MedService;

pub async fn start_scheduler(app_state: Data<AppState>) {
    //0 0 0/8 * * *
    //0 0/5 0 * * *
    let expression = "0 0 0/8 * * *";
    let schedule = Schedule::from_str(expression).unwrap();

    loop {
        let mut upcoming = schedule.upcoming(Utc).take(1);
        actix_rt::time::sleep(Duration::from_millis(500)).await;
        let local = &Local::now();

        if let Some(datetime) = upcoming.next() {
            if datetime.timestamp() <= local.timestamp() {

                log::info!("Running schedule med bot");
                app_state.service.med_service.analyze_appointment(&app_state.client)
                    .await
                    .expect("analyze appointment failed");
            }
        }
    }
}