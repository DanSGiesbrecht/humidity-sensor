extern crate stm32l0xx_hal;
use stm32l0xx_hal::{pac, prelude::*};

let led_1, led_2 = PD<Output<PushPull>>;
