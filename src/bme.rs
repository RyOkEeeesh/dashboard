use bevy::prelude::*;

#[cfg(target_os = "linux")]
use linux_embedded_hal::{Delay, I2cdev};

pub struct MySample {
    pub temperature: f32,
    pub humidity: f32,
    pub pressure: f32,
}
pub struct Bme {
    #[cfg(target_os = "linux")]
    bme280: bme280_rs::Bme280<I2cdev, Delay>,
}

impl Bme {
    pub fn new() -> Result<Self, ()> {
        #[cfg(target_os = "linux")]
        {
            let i2c = I2cdev::new("/dev/i2c-1")?;
            let mut bme280 = bme280_rs::Bme280::new(i2c, Delay);
            bme280.init()?;
            Ok(Self { bme280 })
        }

        #[cfg(not(target_os = "linux"))]
        Err(())
    }

    pub fn read_weather(&mut self) -> Result<MySample, Box<dyn std::error::Error>> {
        #[cfg(target_os = "linux")]
        {
            let data = self.bme280.read_sample()?;
            Ok(MySample {
                temperature: data.temperature,
                humidity: data.humidity,
                pressure: data.pressure,
            })
        }

        #[cfg(not(target_os = "linux"))]
        {
            Ok(MySample {
                temperature: 25.5,
                humidity: 50.0,
                pressure: 1013.25,
            })
        }
    }
}

#[derive(Resource, Default, Debug)]
pub struct WeatherData {
    pub temperature: f32,
    pub humidity: f32,
    pub pressure: f32,
}

// BmeをBevyのリソースとしてラップする
#[derive(Resource)]
struct BmeContainer {
    device: Bme,
}

#[derive(Resource)]
struct BmeTimer(Timer);

pub struct BmePlugin;

impl Plugin for BmePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.init_resource::<WeatherData>();
        app.insert_resource(BmeTimer(Timer::from_seconds(5.0, TimerMode::Repeating)));
        app.add_systems(Startup, setup);
        app.add_systems(Update, read);
    }
}

fn setup(mut commands: Commands) {
    match Bme::new() {
        Ok(bme) => {
            info!("BME280 sensor initialized successfully.");
            // 成功したらリソースとして登録
            commands.insert_resource(BmeContainer { device: bme });
        }
        Err(_) => {
            error!("Failed to initialize BME280. Running in mock mode.");
            // 失敗しても、BmeContainerがないだけでアプリは止まらない
        }
    }
}

fn read(
    time: Res<Time>,
    mut timer: ResMut<BmeTimer>,
    bme_opt: Option<ResMut<BmeContainer>>, // 登録されていない可能性を考慮
    mut weather_data: ResMut<WeatherData>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        if let Some(mut bme_container) = bme_opt {
            if let Ok(sample) = bme_container.device.read_weather() {
                weather_data.temperature = sample.temperature;
                weather_data.humidity = sample.humidity;
                weather_data.pressure = sample.pressure;
            }
        } else {
            // 必要に応じて、リソースがない場合のフォールバック処理
            weather_data.temperature = 0.0;
        }
        println!("5s");
    }
    // リソースが存在する場合のみ実行
}
