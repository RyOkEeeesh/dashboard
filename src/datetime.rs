use crate::ui::{AppStates, DatetimeState, Main};
use chrono::{Local, Timelike};
use slint::{ComponentHandle, Weak};
use tokio_cron_scheduler::{Job, JobScheduler};

pub async fn run_datetime(sched: &JobScheduler, ui: Weak<Main>) {
    sched
        .add(
            Job::new("*/1 * * * * *", move |_id, _lock| {
                let ui_weak = ui.clone();
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
}

pub fn set_datetime(ui: &Main) {
    ui.global::<AppStates>()
        .set_datetime(to_ui_datetime(Local::now()));
}

fn to_ui_datetime(time: chrono::DateTime<Local>) -> DatetimeState {
    DatetimeState {
        h: format!("{:01}", time.hour()).into(),
        m: format!("{:02}", time.minute()).into(),
        s: format!("{:02}", time.second()).into(),
    }
}
