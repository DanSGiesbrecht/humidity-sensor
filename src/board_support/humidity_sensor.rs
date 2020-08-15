use stm32l0xx_hal::{
    gpio::{
        gpiob::{PB13, PB14},
        Output,
        OpenDrain,
        PinMode
    },
    pac::{
        I2C2
    },
    rcc::Rcc,
    i2c::I2c,
    delay::Delay,
    prelude::*,
};

use shtcx::{ShtC3, shtc3, PowerMode, Measurement, LowPower};

pub struct HumiditySensor {
    sht: ShtC3<I2c<I2C2, PB14<Output<OpenDrain>>, PB13<Output<OpenDrain>>>>
}

impl HumiditySensor {
    pub fn new<T: PinMode>(sda_pin: PB14<T>, scl_pin: PB13<T>, i2c_peripheral: I2C2, rcc: &mut Rcc) -> Self {
        let sda = sda_pin.into_open_drain_output();
        let scl = scl_pin.into_open_drain_output();
        HumiditySensor{sht: shtc3(i2c_peripheral.i2c(sda, scl, 100.khz(), rcc))}
    }

    pub fn read(&mut self, delay: &mut Delay) -> Measurement {
        self.sht.wakeup(delay).unwrap();
        let measurement = self.sht.measure(PowerMode::LowPower, delay).unwrap();
        self.sht.sleep().unwrap();
        measurement
    }
}

