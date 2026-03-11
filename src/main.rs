use linux_embedded_hal::{I2cdev, Delay};
use bme280_rs::{Bme280, Configuration, Oversampling, SensorMode};
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let i2c = I2cdev::new("/dev/i2c-1")?;
    let delay = Delay;

    let mut bme280 = Bme280::new(i2c, delay); 

    bme280.init()?;

    let config = Configuration::default()
        .with_temperature_oversampling(Oversampling::Oversample1)
        .with_humidity_oversampling(Oversampling::Oversample1)
        .with_pressure_oversampling(Oversampling::Oversample1)
        .with_sensor_mode(SensorMode::Normal);

    // & を消して config をそのまま渡す
    bme280.set_sampling_configuration(config)?;

    println!("Rustで計測を開始します（5秒おき）...");

    loop {
        match bme280.read_sample() {
            Ok(data) => {
                println!(
                    "温度: {:.2} °C, 湿度: {:.2} %, 気圧: {:.2} hPa",
                    data.temperature.unwrap_or(0.0),
                    data.humidity.unwrap_or(0.0),
                    data.pressure.unwrap_or(0.0) / 100.0
                );
            }
            Err(e) => eprintln!("読み取りエラー: {:?}", e),
        }

        thread::sleep(Duration::from_secs(5));
    }
}