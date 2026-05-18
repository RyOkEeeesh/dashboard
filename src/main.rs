use dashboard::datetime::{run_datetime, set_datetime};
use dashboard::entities::room_temp;
use dashboard::home::setup_home;
use slint::ComponentHandle;

use chrono::{Local, Timelike};
use std::sync::Arc;
use tokio::sync::{Mutex, mpsc, oneshot};
use tokio_cron_scheduler::JobScheduler;

use dashboard::bme::{Bme, run_bme};
use dashboard::db::{Date, DbRequest, db_run};
use dashboard::ui::{AppStates, Main};

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel::<DbRequest>(100);

    db_run(rx);

    let ui: Main = Main::new().unwrap();

    let bme_result = Bme::new();
    if bme_result.is_ok() {
        ui.global::<AppStates>().set_can_bme_use(true);
    }

    let bme_mod = Arc::new(Mutex::new(bme_result));

    // init
    set_datetime(&ui);
    setup_home(tx.clone(), &ui).await;

    get_temp_data(tx.clone(), Date(2026, 5, 16)).await;

    //
    wait().await;

    let sched: JobScheduler = JobScheduler::new().await.unwrap();
    let ui_weak = ui.as_weak();

    // datetime
    run_datetime(&sched, ui_weak.clone()).await;

    // BME
    run_bme(&sched, ui_weak.clone(), bme_mod.clone(), tx.clone()).await;

    sched.start().await.unwrap();
    ui.run().unwrap();
}

async fn wait() {
    let sleep_ms = (1_000_000_000 - Local::now().nanosecond()) / 1_000_000;
    if sleep_ms > 0 {
        tokio::time::sleep(std::time::Duration::from_millis(sleep_ms as u64)).await;
    }
}

async fn get_temp_data(tx: mpsc::Sender<DbRequest>, date: Date) {
    let (tx_oneshot, rx_oneshot) = oneshot::channel::<Vec<room_temp::Model>>();
    tx.send(DbRequest::GetTemp(tx_oneshot, date)).await.unwrap();

    match rx_oneshot.await {
        Ok(data) => {
            dbg!(data);
        }
        Err(e) => eprint!("{e}"),
    }
}
