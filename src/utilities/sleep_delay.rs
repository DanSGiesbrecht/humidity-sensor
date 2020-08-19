use embedded_hal::{
    blocking::delay::{
        DelayMs,
        DelayUs,
    },
    timer::CountDown,
};

use stm32l0xx_hal::{
    lptim::{
        LpTimer,
        OneShot,
        ClockSrc,
        Interrupts
    },
    pac::{
        LPTIM,
    },
    pwr::PWR,
    rcc::Rcc,
    time::MicroSeconds,
    prelude::*,
};

use cortex_m::asm::wfi;

pub struct SleepDelay{
    timer: LpTimer<OneShot>
}

impl SleepDelay {
    pub fn new(lptim: LPTIM, pwr: &mut PWR, rcc: &mut Rcc) -> Self {
        SleepDelay{timer: LpTimer::init_oneshot(lptim, pwr, rcc, ClockSrc::Lsi)}
    }

    pub fn delay<T>(&mut self, delay: T) where T: Into<MicroSeconds> {
        self.delay_us(delay.into().0 as u64)
    }

    fn delay_ms(&mut self, ms: u64) {
        self.delay_us(ms * 1000);
    }

    fn delay_us(&mut self, us: u64) {
        const INTERRUPTS: Interrupts = Interrupts {
            enc_dir_down: false,
            enc_dir_up: false,
            autoreload_update_ok: false,
            compare_update_ok: false,
            ext_trig: false,
            autoreload_match: false,
            compare_match: true
        };

        self.timer.enable_interrupts(INTERRUPTS);
        self.timer.start((us as u32).us());

        while let Err(_) = self.timer.wait() {
            wfi();
        }

        self.timer.enable_interrupts(INTERRUPTS);
    }
}

impl DelayUs<u32> for SleepDelay {
    fn delay_us(&mut self, us: u32) {
        self.delay_us(us as u64);
    }
}

impl DelayUs<u16> for SleepDelay {
    fn delay_us(&mut self, us: u16) {
        self.delay_us(us as u64);
    }
}

impl DelayUs<u8> for SleepDelay {
    fn delay_us(&mut self, us: u8) {
        self.delay_us(us as u64);
    }
}

impl DelayMs<u32> for SleepDelay {
    fn delay_ms(&mut self, us: u32) {
        self.delay_ms(us as u64);
    }
}

impl DelayMs<u16> for SleepDelay {
    fn delay_ms(&mut self, us: u16) {
        self.delay_ms(us as u64);
    }
}

impl DelayMs<u8> for SleepDelay {
    fn delay_ms(&mut self, us: u8) {
        self.delay_ms(us as u64);
    }
}
