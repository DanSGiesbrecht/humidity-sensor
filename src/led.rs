extern crate stm32l0xx_hal;
use stm32l0xx_hal::{pac, prelude::*, rcc::Config};

pub fn led_test() -> () {
    let periph = pac::Peripherals::take().unwrap();
    let mut rcc = periph.RCC.freeze(Config::hsi16());

    let gpioa = periph.GPIOA.split(&mut rcc);

    let mut led1 = gpioa.pa0.into_push_pull_output();
    let mut led2 = gpioa.pa1.into_push_pull_output();
    led1.set_high().unwrap();
    led2.set_high().unwrap();
}