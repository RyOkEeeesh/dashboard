#[cfg(target_os = "linux")]
use bme280_rs::Bme280;
#[cfg(target_os = "linux")]
use linux_embedded_hal::{Delay, I2cdev};

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
            let i2c = I2cdev::new("/dev/i2c-1")?;
            let mut bme280 = Bme280::new(i2c, Delay);
            bme280.init()?;
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
                temp: data.temp,
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
