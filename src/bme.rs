use bme280_rs::{Bme280, Configuration, Oversampling, SensorMode};
use linux_embedded_hal::{Delay, I2cdev};
use anyhow::Result; // エラーハンドリングを楽にするため（cargo add anyhow してね）

pub struct Bme {
    // Bme280型はジェネリクスを持っているので、使用しているI2CとDelayの型を指定します
    bme280: Bme280<I2cdev, Delay>,
}

impl Bme {
    // &self を取るのではなく、自分自身（Self）を返す「コンストラクタ」にします
    pub fn new() -> Result<Self> {
        let i2c = I2cdev::new("/dev/i2c-1")
            .map_err(|e| anyhow::anyhow!("I2Cのオープンに失敗: {}", e))?;
        let delay = Delay;

        let mut bme280 = Bme280::new(i2c, delay);
        bme280.init()?; // ここでセンサーの初期化

        // 完成したBme構造体を返す
        Ok(Self { bme280 })
    }

    // 実際に計測するメソッドも作っておくと便利
    pub fn read_weather(&mut self) -> Result<()> {
        let data = self.bme280.read_sample()?;
        println!("温度: {:.2} °C, 湿度: {:.2} %", data.temperature, data.humidity);
        Ok(())
    }
}