use std::sync::Arc;

#[cfg(target_os = "linux")]
use bme280_rs::{Bme280, Configuration, Oversampling, SensorMode};
#[cfg(target_os = "linux")]
use linux_embedded_hal::{Delay, I2cdev};

use crate::{
    db::DbRequest,
    ui::{AppStates, Main, WeatherDataUi},
};
use slint::{ComponentHandle, Weak};
use tokio::sync::{Mutex, mpsc::Sender};
use tokio_cron_scheduler::{Job, JobScheduler};

#[derive(Clone)]
pub struct WeatherData {
    pub temp: Option<f32>,
    pub humidity: Option<f32>,
    pub pressure: Option<f32>,
}

pub struct Bme {
    #[cfg(target_os = "linux")]
    bme280: Bme280<I2cdev, Delay>,
}

impl Bme {
    pub fn new() -> Result<Self, ()> {
        #[cfg(target_os = "linux")]
        {
            let i2c = I2cdev::new("/dev/i2c-1").map_err(|_| ())?;
            let mut bme280 = Bme280::new(i2c, Delay);

            bme280.init().map_err(|_| ())?;

            let config = Configuration::default()
                .with_temperature_oversampling(Oversampling::Oversample1)
                .with_humidity_oversampling(Oversampling::Oversample1)
                .with_pressure_oversampling(Oversampling::Oversample1)
                .with_sensor_mode(SensorMode::Normal);

            bme280.set_sampling_configuration(config).map_err(|_| ())?;

            Ok(Self { bme280 })
        }

        #[cfg(not(target_os = "linux"))]
        Err(())
    }

    pub fn read_weather(&mut self) -> Result<WeatherData, Box<dyn std::error::Error>> {
        #[cfg(target_os = "linux")]
        {
            let data = self.bme280.read_sample()?;
            Ok(WeatherData {
                temp: data.temperature,
                humidity: data.humidity,
                pressure: data.pressure,
            })
        }

        #[cfg(not(target_os = "linux"))]
        {
            Ok(WeatherData {
                temp: None,
                humidity: None,
                pressure: None,
            })
        }
    }
}

pub async fn run_bme(
    sched: &JobScheduler,
    ui: Weak<Main>,
    bme: Arc<Mutex<Result<Bme, ()>>>,
    tx: Sender<DbRequest>,
) {
    sched
        .add(
            Job::new("*/5 * * * * *", move |_id, _lock| {
                let ui_weak = ui.clone();
                let bme_lock = bme.clone();
                let tx = tx.clone();

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
                    }
                });
            })
            .unwrap(),
        )
        .await
        .unwrap();
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
