#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;
extern crate shtcx;

// use patch
use stm32l0xx_hal::{pac, prelude::*, rcc::Config};

#[entry]
fn main() -> ! {
    let periph = pac::Peripherals::take().unwrap();

    let mut rcc = periph.RCC.freeze(Config::hsi16());
    let gpiob = periph.GPIOB.split(&mut rcc);

    let sda = gpiob.pb14.into_open_drain_output();
    let scl = gpiob.pb13.into_open_drain_output();

    let mut i2c = periph.I2C2.i2c(sda, scl, 10.khz(), &mut rcc);

    let mut buffer = [0u8; 2];
    const ADDRESS: u8 = 0x70;

    i2c.write(ADDRESS, &mut buffer).unwrap();

    loop {
    }
}

