#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;
use stm32l0xx_hal::{
    pac::{
        Peripherals,
        CorePeripherals,
    },
    rcc::Config,
    prelude::*,
    delay::Delay
};

mod board_support;
use board_support::{
    humidity_sensor::HumiditySensor,
    rf_transmitter::RfTransmitter,
    random_number_generator::RandomNumberGenerator
};

#[entry]
fn main() -> ! {
    let (core_periph, periph) = (CorePeripherals::take().unwrap(), Peripherals::take().unwrap());

    let mut rcc = periph.RCC.freeze(Config::hsi16());
    let gpiob = periph.GPIOB.split(&mut rcc);

    let mut delay = Delay::new(core_periph.SYST, rcc.clocks);
    let mut humidity_sensor = HumiditySensor::new(gpiob.pb14, gpiob.pb13, periph.I2C2, &mut rcc);
    let mut disabled_transmitter = RfTransmitter::new();

    let mut random_number_generator = RandomNumberGenerator::new(
        humidity_sensor.read(&mut delay).to_combined_u64()
    );

    loop {
        delay.delay(random_number_generator.next(500, 2000).ms());

        let measurement = humidity_sensor.read(&mut delay);

        let mut enabled_transmitter = disabled_transmitter.enable(&mut delay);
        enabled_transmitter.send(&measurement.to_combined_array());
        disabled_transmitter = enabled_transmitter.disable(&mut delay);
    }
}

trait MeasurementExt {
    fn to_combined_array(&self) -> [u8; 8];
    fn to_combined_u64(&self) -> u64;
}

impl MeasurementExt for shtcx::Measurement {
    fn to_combined_array(&self) -> [u8; 8] {
        self.to_combined_u64().to_le_bytes()
    }

    fn to_combined_u64(&self) -> u64 {
        let temperature = self.temperature.as_millidegrees_celsius();
        let humidity = self.humidity.as_millipercent();

        (temperature as u64) | ((humidity as u64) << 32)
    }
}
