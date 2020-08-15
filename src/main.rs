#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;

use stm32l0xx_hal::{
    gpio::{
        gpiob::{PB13, PB14},
        Output,
        OpenDrain
    },
    pac::{
        Peripherals,
        CorePeripherals,
        I2C2
    },
    i2c::I2c,
    prelude::*,
    rcc::Config,
    delay::Delay
};

use shtcx::{ShtC3, shtc3, PowerMode, Measurement, LowPower};

type Sht = ShtC3<I2c<I2C2, PB14<Output<OpenDrain>>, PB13<Output<OpenDrain>>>>;

#[entry]
fn main() -> ! {
    let periph = Peripherals::take().unwrap();
    let core_periph = CorePeripherals::take().unwrap();

    let mut rcc = periph.RCC.freeze(Config::hsi16());
    let gpiob = periph.GPIOB.split(&mut rcc);

    // Configure I2C for SHTC3
    let sda = gpiob.pb14.into_open_drain_output();
    let scl = gpiob.pb13.into_open_drain_output();
    let i2c = periph.I2C2.i2c(sda, scl, 100.khz(), &mut rcc);

    let mut delay = Delay::new(core_periph.SYST, rcc.clocks);
    let mut sht = shtc3(i2c);

    loop {
        // TODO: Generate sleep time randomly

        delay.delay(2000.ms());

        let _ = get_humidity_temperature_measurement(&mut sht, &mut delay);

        // TODO: Turn on RF section
        
        // TODO: Sleep for requisite time for RF to get ready

        // TODO: Load UART/DMA buffer from measurement
        
        // TODO: Sleep until buffer will be done sending
        
        // TODO: Turn off RF section
    }
}

fn get_humidity_temperature_measurement(sht: &mut Sht, delay: &mut Delay) -> Measurement {
    sht.wakeup(delay).unwrap();
    let measurement = sht.measure(PowerMode::LowPower, delay).unwrap();
    sht.sleep().unwrap();
    measurement
}
