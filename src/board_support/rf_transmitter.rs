use stm32l0xx_hal::{
    delay::Delay,
    prelude::*,
    spi,
    spi::*,
    rcc::Rcc,
    pac::SPI1,
    gpio::{
        gpioa::{PA6, PA7},
        Output,
        PushPull,
        PinMode,
        Analog
    }
};

use core::marker::PhantomData;

pub struct RfTransmitter<ENABLED> {
    _marker:        PhantomData<ENABLED>,
    rf_enable:      PA6<Output<PushPull>>,
    rf_spi:         spi::Spi<SPI1, (NoSck, NoMiso, PA7<Analog>)>
}

pub struct Enabled;
pub struct Disabled;

impl RfTransmitter<Disabled> {
    pub fn new<T: PinMode>(rf_enable_pin: PA6<T>, mosi: PA7<T>, spi_periph: SPI1, rcc: &mut Rcc) -> RfTransmitter<Disabled> {
        let mut rf_enable_pin = rf_enable_pin.into_push_pull_output();
        rf_enable_pin.set_low().unwrap();
        let rf_spi = spi_periph.spi((spi::NoSck, spi::NoMiso, mosi.into_analog()), spi::MODE_0, 50_000.hz(), rcc);

        RfTransmitter{
            _marker:    PhantomData,
            rf_enable:  rf_enable_pin,
            rf_spi:     rf_spi
        }
    }

    pub fn enable(mut self, delay: &mut Delay) -> RfTransmitter<Enabled> {
        self.rf_enable.set_high().unwrap();

        // TODO Determine correct amount of time to wait for stabilization
        delay.delay(100.ms());

        RfTransmitter{
            _marker:    PhantomData,
            rf_enable:  self.rf_enable,
            rf_spi:        self.rf_spi
        }
    }
}

impl RfTransmitter<Enabled> {
    pub fn disable(mut self, delay: &mut Delay) -> RfTransmitter<Disabled> {
        // TODO: Delay until ongoing transmission is complete rather
        // than just an arbitrary wait
        delay.delay(100.ms());

        self.rf_enable.set_low().unwrap();

        RfTransmitter{
            _marker:    PhantomData,
            rf_enable:  self.rf_enable,
            rf_spi:        self.rf_spi
        }
    }

    pub fn send(&mut self, _data: &[u8]) {
        // TODO: Load transmit buffer and start transmission
        self.rf_spi.write(_data).unwrap();
    }
}
