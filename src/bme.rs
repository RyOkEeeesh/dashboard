pub struct WeatherData {
    pub temperature: Option<f32>,
    pub humidity: Option<f32>,
    pub pressure: Option<f32>,
}

pub struct Bme {
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
