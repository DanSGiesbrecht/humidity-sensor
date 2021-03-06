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
    _marker:    PhantomData<ENABLED>,
    enable:     PA6<Output<PushPull>>,
    spi:        spi::Spi<SPI1, (NoSck, NoMiso, PA7<Analog>)>
}

pub struct Enabled;
pub struct Disabled;

impl RfTransmitter<Disabled> {
    pub fn new<T: PinMode>(enable_pin: PA6<T>, mosi: PA7<T>, spi_periph: SPI1, rcc: &mut Rcc) -> Self {
        let mut transmitter = RfTransmitter{
            _marker:    PhantomData,
            enable:     enable_pin.into_push_pull_output(),
            spi:        spi_periph.spi(
                (spi::NoSck, spi::NoMiso, mosi.into_analog()),
                spi::MODE_0,
                50_000.hz(),
                rcc
            )
        };

        transmitter.enable.set_low().unwrap();

        transmitter
    }

    pub fn enable(mut self, delay: &mut Delay) -> RfTransmitter<Enabled> {
        self.enable.set_high().unwrap();

        // TODO Determine correct amount of time to wait for stabilization
        delay.delay(100.ms());

        RfTransmitter{
            _marker:    PhantomData,
            enable:     self.enable,
            spi:        self.spi
        }
    }
}

impl RfTransmitter<Enabled> {
    pub fn disable(mut self, delay: &mut Delay) -> RfTransmitter<Disabled> {
        // TODO: Delay until ongoing transmission is complete rather
        // than just an arbitrary wait
        delay.delay(100.ms());

        self.enable.set_low().unwrap();

        RfTransmitter{
            _marker:    PhantomData,
            enable:     self.enable,
            spi:        self.spi
        }
    }

    pub fn send(&mut self, data: &[u8]) {
        // TODO: Load transmit buffer and start transmission
        self.spi.write(data).unwrap();
    }
}
