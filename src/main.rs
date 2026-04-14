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
                let ui_weak = ui_weak.clone(); // クロージャ内で使うため

                tokio::spawn(async move {
                    let now = Local::now();

                    // 1. 次の「0秒」までの時間を計算する
                    // 1秒(1,000,000,000ナノ秒) - 現在のナノ秒
                    let nanos = now.nanosecond();
                    let sleep_ms = (1_000_000_000 - nanos) / 1_000_000;

                    // 2. その差分だけ待つ（これでOSの時計と同期する）
                    if sleep_ms > 0 {
                        tokio::time::sleep(std::time::Duration::from_millis(sleep_ms as u64)).await;
                    }

                    // 3. ちょうど「秒」が変わったタイミングで時刻を取得してUI更新
                    let sync_now = Local::now();
                    let state = DatetimeState {
                        Y: sync_now.year() as i32,
                        M: sync_now.month() as i32,
                        D: sync_now.day() as i32,
                        h: sync_now.hour() as i32,
                        m: sync_now.minute() as i32,
                        s: sync_now.second() as i32,
                    };

                    ui_weak
                        .upgrade_in_event_loop(move |ui| {
                            ui.set_datetime(state);
                        })
                        .ok();
                });
            })
            .unwrap(),
        )
        .await
        .unwrap();

    sched.start().await.unwrap();
    ui.run().unwrap();
}
