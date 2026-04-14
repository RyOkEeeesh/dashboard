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
                let ui_weak = ui_weak.clone();

                tokio::spawn(async move {
                    let now = Local::now();

                    ui_weak
                        .upgrade_in_event_loop(move |ui| {
                            ui.set_datetime(to_ui_datetime(now));
                        })
                        .ok();

                    // 1. 次の「0秒」までの時間を計算する
                    // 1秒(1,000,000,000ナノ秒) - 現在のナノ秒
                    let nanos = now.nanosecond();
                    let sleep_ms = (1_000_000_000 - nanos) / 1_000_000;

                    if sleep_ms > 0 {
                        tokio::time::sleep(std::time::Duration::from_millis(sleep_ms as u64)).await;
                    }

                    // 3. ちょうど「秒」が変わったタイミングで時刻を取得してUI更新
                    let state= to_ui_datetime(Local::now());

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

fn to_ui_datetime(time: chrono::DateTime<Local>) -> DatetimeState {
    DatetimeState {
        Y: format!("{}", time.year()).into(),
        M: format!("{:02}", time.month()).into(),
        D: format!("{:02}", time.day()).into(),
        h: format!("{:02}", time.hour()).into(),
        m: format!("{:02}", time.minute()).into(),
        s: format!("{:02}", time.second()).into(),
    }
}