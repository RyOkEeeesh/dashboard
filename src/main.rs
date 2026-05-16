mod ui {
    slint::include_modules!();
}
use dashboard::entities::room_temp;
use slint::Model;
use ui::*;

use chrono::{Local, Timelike};
use std::sync::Arc;
use tokio::sync::{Mutex, mpsc, oneshot};
use tokio_cron_scheduler::{Job, JobScheduler};

use dashboard::bme::{Bme, WeatherData};
use dashboard::db::{Date, DbRequest, db_run};

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel::<DbRequest>(100);

    db_run(rx);

    let ui: Main = Main::new().unwrap();
    let ui_weak = ui.as_weak();

    let bme_result = Bme::new();
    if bme_result.is_ok() {
        ui.global::<AppStates>().set_can_bme_use(true);
    }

    let bme_mod = Arc::new(Mutex::new(bme_result));

    // init
    set_datetime(&ui);
    apps_alignment(&ui);

    let sleep_ms = (1_000_000_000 - Local::now().nanosecond()) / 1_000_000;
    if sleep_ms > 0 {
        tokio::time::sleep(std::time::Duration::from_millis(sleep_ms as u64)).await;
    }

    let sched = JobScheduler::new().await.unwrap();

    // datetime
    let ui_weak_job_datetime = ui_weak.clone();

    get_temp_data(tx.clone(), Date(2026, 5, 16)).await;

    sched
        .add(
            Job::new("*/1 * * * * *", move |_id, _lock| {
                let ui_weak = ui_weak_job_datetime.clone();
                tokio::spawn(async move {
                    ui_weak
                        .upgrade_in_event_loop(move |ui| set_datetime(&ui))
                        .ok();
                });
            })
            .unwrap(),
        )
        .await
        .unwrap();

    // BME
    let ui_weak_job_bme = ui_weak.clone();
    let bme_for_job = bme_mod.clone();
    let tx_for_job = tx.clone();

    sched
        .add(
            Job::new("*/5 * * * * *", move |_id, _lock| {
                let bme_lock = bme_for_job.clone();
                let ui_weak = ui_weak_job_bme.clone();
                let tx = tx_for_job.clone();

                tokio::spawn(async move {
                    if let Ok(ref mut bme) = *bme_lock.lock().await {
                        if let Ok(result) = bme.read_weather() {
                            let ui_result = result.clone();
                            ui_weak
                                .upgrade_in_event_loop(move |ui| {
                                    ui.global::<AppStates>()
                                        .set_bme_result(to_ui_weather_data(ui_result))
                                })
                                .ok();
                            let _ = tx.try_send(DbRequest::SetTemp(result));
                        }
                    } else {
                        let result = WeatherData {
                            temp: Some(15.6),
                            humidity: Some(32.0),
                            pressure: Some(1013.2),
                        };
                        let _ = tx.try_send(DbRequest::SetTemp(result));
                    }
                });
            })
            .unwrap(),
        )
        .await
        .unwrap();

    sched.start().await.unwrap();
    ui.run().unwrap();

}

async fn get_temp_data(tx: mpsc::Sender<DbRequest>, date: Date) {
    let (tx_oneshot, rx_oneshot) = oneshot::channel::<Vec<room_temp::Model>>();
    tx.send(DbRequest::GetTemp(tx_oneshot, date)).await.unwrap();

    match rx_oneshot.await {
        Ok(data) => {
            dbg!(data);
        },
        Err(e) => eprint!("{e}"),
    }
}

fn set_datetime(ui: &Main) {
    ui.global::<AppStates>()
        .set_datetime(to_ui_datetime(Local::now()));
}

fn apps_alignment(ui: &Main) {
    let apps: Vec<AppData> = ui.global::<AppStates>().get_apps().iter().collect();
}

fn to_ui_datetime(time: chrono::DateTime<Local>) -> DatetimeState {
    DatetimeState {
        h: format!("{:01}", time.hour()).into(),
        m: format!("{:02}", time.minute()).into(),
        s: format!("{:02}", time.second()).into(),
    }
}

fn to_ui_weather_data(wd: WeatherData) -> WeatherDataUi {
    WeatherDataUi {
        temp: wd
            .temp
            .map(|t| format!("{:.1}", t))
            .unwrap_or_else(|| "--".to_string())
            .into(),
        humidity: wd
            .humidity
            .map(|t| format!("{:.1}", t))
            .unwrap_or_else(|| "--".to_string())
            .into(),
        pressure: wd
            .pressure
            .map(|t| format!("{:.1}", t))
            .unwrap_or_else(|| "--".to_string())
            .into(),
    }
}
