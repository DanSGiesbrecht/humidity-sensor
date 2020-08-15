use stm32l0xx_hal::{
    delay::Delay,
    prelude::*,
};

use core::marker::PhantomData;

pub struct RfTransmitter<ENABLED> {
    _marker: PhantomData<ENABLED>
}

pub struct Enabled;
pub struct Disabled;

impl RfTransmitter<Disabled> {
    pub fn new() -> RfTransmitter<Disabled> {
        RfTransmitter{ _marker: PhantomData }
    }

    pub fn enable(&mut self, delay: &mut Delay) -> RfTransmitter<Enabled> {
        // TODO Turn on transmitter circuit

        // TODO Determine correct amount of time to wait for stabilization
        delay.delay(100.ms());

        RfTransmitter{ _marker: PhantomData }
    }
}

impl RfTransmitter<Enabled> {
    pub fn disable(&mut self, delay: &mut Delay) -> RfTransmitter<Disabled> {
        // TODO: Delay until ongoing transmission is complete rather
        // than just an arbitrary wait
        delay.delay(100.ms());

        // TODO: Turn off the transmitter circuit

        RfTransmitter{ _marker: PhantomData }
    }

    pub fn send(&mut self, _data: &[u8]) {
        // TODO: Load transmit buffer and start transmission
    }
}

