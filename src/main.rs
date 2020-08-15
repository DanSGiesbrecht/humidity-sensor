#![no_std]
#![no_main]

extern crate panic_halt;
extern crate shtcx;

use cortex_m_rt::entry;

// use patch for I2C alternate functions
use stm32l0xx_hal::{pac, prelude::*, rcc::Config, delay::Delay};

use shtcx::{ShtCx, PowerMode};

#[entry]
fn main() -> ! {
    let periph = pac::Peripherals::take().unwrap();
    let core_periph = pac::CorePeripherals::take().unwrap();

    let mut rcc = periph.RCC.freeze(Config::hsi16());
    let gpiob = periph.GPIOB.split(&mut rcc);

    // Configure I2C for SHTC3
    let sda = gpiob.pb14.into_open_drain_output();
    let scl = gpiob.pb13.into_open_drain_output();

    let mut i2c = periph.I2C2.i2c(sda, scl, 100.khz(), &mut rcc);
    let delay = Delay::new(core_periph.SYST, rcc.clocks);

    const ADDRESS: u8 = 0x70;
    let mut sht = ShtCx::new(i2c, ADDRESS, delay);

    // add error-handling for unwraps, probably reset sensor
    loop {
        // Calculate time until next transmit, delay [and sleep] for that time

        // Take a temperature and humidity measurement
        sht.wakeup().unwrap(); 
        let measurement = sht.measure(PowerMode::NormalMode).unwrap();
        let milli_temp = measurement.temperature.as_millidegrees_celsius();
        let milli_hum = measurement.humidity.as_millipercent();
        sht.sleep();

        // Transmit the measurements (use DMA, or busy-wait)
    }
}

fn calculate_tx_delay_ms() -> u32 {
    1000;
}