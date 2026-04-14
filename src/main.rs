slint::include_modules!();
use chrono::{Datelike, Local, Timelike};
use tokio_cron_scheduler::{Job, JobScheduler};

#[tokio::main]
async fn main() {
    let ui = App::new().unwrap();
    let ui_weak = ui.as_weak();

    let sched = JobScheduler::new().await.unwrap();

    // datetime
    sched
        .add(
            Job::new("*/1 * * * * *", move |_id, _lock| {
                let now = Local::now();
                let state = DatetimeState {
                    Y: now.year() as i32,
                    M: now.month() as i32,
                    D: now.day() as i32,
                    h: now.hour() as i32,
                    m: now.minute() as i32,
                    s: now.second() as i32,
                };

                // ui_weak から直接呼び出す
                // これ自体が「もし生きていればメインスレッドで実行する」という処理になります
                ui_weak
                    .upgrade_in_event_loop(move |ui| {
                        ui.set_datetime(state);
                    })
                    .expect("Failed to update UI");
            })
            .unwrap(),
        )
        .await
        .unwrap();

    sched.start().await.unwrap();
    ui.run().unwrap();
}
