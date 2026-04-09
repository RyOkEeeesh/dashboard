use crate::{clock::SecondTick, db::{DbRequest, DbSender}};
use bevy::prelude::*;

#[cfg(target_os = "linux")]
use linux_embedded_hal::{Delay, I2cdev};

#[derive(Resource, Default, Clone, Debug)]
pub struct WeatherData {
    pub temperature: Option<f32>,
    pub humidity: Option<f32>,
    pub pressure: Option<f32>,
}

struct Bme {
    #[cfg(target_os = "linux")]
    bme280: bme280_rs::Bme280<I2cdev, Delay>,
}

impl Bme {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(target_os = "linux")]
        {
            let i2c = I2cdev::new("/dev/i2c-1")?;
            let mut bme280 = bme280_rs::Bme280::new(i2c, Delay);
            bme280.init()?;
            Ok(Self { bme280 })
        }

        #[cfg(not(target_os = "linux"))]
        Err("Unsupported platform".into())
    }

    pub fn read_weather(&mut self) -> Result<WeatherData, Box<dyn std::error::Error>> {
        #[cfg(target_os = "linux")]
        {
            let data = self.bme280.read_sample()?;
            Ok(WeatherData {
                temperature: data.temperature,
                humidity: data.humidity,
                pressure: data.pressure,
            })
        }

        #[cfg(not(target_os = "linux"))]
        {
            Ok(WeatherData {
                temperature: None,
                humidity: None,
                pressure: None,
            })
        }
    }
}

#[derive(Resource)]
struct BmeContainer(Bme);

pub struct BmePlugin;

impl Plugin for BmePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WeatherData>()
            .add_systems(Startup, setup)
            .add_observer(on_read);
    }
}

fn setup(mut commands: Commands) {
    match Bme::new() {
        Ok(bme) => {
            info!("BME280 sensor initialized successfully.");
            commands.insert_resource(BmeContainer(bme));
        }
        Err(_) => {
            error!("Failed to initialize BME280. Running in mock mode.");
        }
    }
}

fn on_read(
    trigger: On<SecondTick>,
    bme: Option<ResMut<BmeContainer>>,
    mut weather_data: ResMut<WeatherData>,
    db_sender: Res<DbSender>,
) {
    if trigger.event().0 % 5 == 0 {
        if let Some(mut bme_container) = bme {
            if let Ok(sample) = bme_container.0.read_weather() {
                *weather_data = sample.clone();
                let _ = db_sender.0.try_send(DbRequest::SaveWeather(sample));
            }
        }
        let _ = db_sender.0.try_send(DbRequest::SaveWeather(WeatherData {
            temperature: Some(15.0),
            humidity: Some(50.0),
            pressure: Some(1013.25),
        }));
        let now = chrono::Local::now();
        println!("{now}");
    }
}
