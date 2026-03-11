import board
import busio
# ここを書き換えます
from adafruit_bme280.basic import Adafruit_BME280_I2C

# I2C通信の初期化
i2c = busio.I2C(board.SCL, board.SDA)

# 呼び出し時、頭の「adafruit_bme280.」は不要になります
bme280 = Adafruit_BME280_I2C(i2c, address=0x76) # 0x76が多いです

# 計測結果を表示
print(f"Temperature: {bme280.temperature:.2f} °C")
print(f"Humidity: {bme280.humidity:.2f} %")
print(f"Pressure: {bme280.pressure:.2f} hPa")