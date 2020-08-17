use stm32l0xx_hal::{
    delay::Delay,
    prelude::*,
    gpio::{
        gpioa::{PA6},
        Output,
        PushPull,
        PinMode
    }
};

use core::marker::PhantomData;

pub struct RfTransmitter<ENABLED> {
    _marker:        PhantomData<ENABLED>,
    rf_enable_pin:  PA6<Output<PushPull>>
}

pub struct Enabled;
pub struct Disabled;

impl RfTransmitter<Disabled> {
    pub fn new<T: PinMode>(rf_enable_pin: PA6<T>) -> RfTransmitter<Disabled> {
        let mut enable = rf_enable_pin.into_push_pull_output();
        enable.set_low().unwrap();

        RfTransmitter{
            _marker: PhantomData,
            rf_enable_pin: enable
        }
    }

    pub fn enable(mut self, delay: &mut Delay) -> RfTransmitter<Enabled> {
        self.rf_enable_pin.set_high().unwrap();

        // TODO Determine correct amount of time to wait for stabilization
        delay.delay(100.ms());

        RfTransmitter{
            rf_enable_pin: self.rf_enable_pin,
            _marker: PhantomData
        }
    }
}

impl RfTransmitter<Enabled> {
    pub fn disable(mut self, delay: &mut Delay) -> RfTransmitter<Disabled> {
        // TODO: Delay until ongoing transmission is complete rather
        // than just an arbitrary wait
        delay.delay(100.ms());

        self.rf_enable_pin.set_low().unwrap();

        RfTransmitter{
            rf_enable_pin: self.rf_enable_pin,
            _marker: PhantomData
        }
    }

    pub fn send(&mut self, _data: &[u8]) {
        // TODO: Load transmit buffer and start transmission
    }
}

