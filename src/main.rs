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
use board_support::humidity_sensor::HumiditySensor;

#[entry]
fn main() -> ! {
    let (core_periph, periph) = (CorePeripherals::take().unwrap(), Peripherals::take().unwrap());

    let mut rcc = periph.RCC.freeze(Config::hsi16());
    let gpiob = periph.GPIOB.split(&mut rcc);

    let mut delay = Delay::new(core_periph.SYST, rcc.clocks);
    let mut humidity_sensor = HumiditySensor::new(gpiob.pb14, gpiob.pb13, periph.I2C2, &mut rcc);

    loop {
        // TODO: Generate sleep time randomly

        delay.delay(2000.ms());

        let _ = humidity_sensor.read(&mut delay);

        // TODO: Turn on RF section
        
        // TODO: Sleep for requisite time for RF to get ready

        // TODO: Load UART/DMA buffer from measurement
        
        // TODO: Sleep until buffer will be done sending
        
        // TODO: Turn off RF section
    }
}
