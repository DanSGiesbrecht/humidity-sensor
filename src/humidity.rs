extern crate shtcx;
use shtcx::{ShtCx, PowerMode};

/***************************************/
// I HAVE SOME LEARNIN' TO DO ON TRAITS!!
/***************************************/

pub struct Humtemp {
    sensor:             ShtCx<I, A, D>,
    // can remove service_needed? Don't need it?
    service_needed:     bool,
    temperature:        f32, // replace with buffer
    humidity:           f32, // replace with buffer
}

impl Humtemp {
    pub fn new<I, A, D>(sensor: ShtCx<I, A, D>) -> Self {
        Self {
            sensor:         sensor,
    // can remove service_needed? Don't need it?
            service_needed: false,
            temperature:    0.0, // placeholder
            humidity:       0.0, // placeholder
        }
    }

    // can remove service_needed? Don't need it?
    pub fn flag(&mut self) {
        self.service_needed = true;
    }

    pub fn service(&mut self) {
    // can remove service_needed? Don't need it?
        if self.service_needed == true {
            self.sensor.wakeup().unwrap();
            let meas            = self.sensor.measure(PowerMode::LowPowerMode).unwrap();
            self.temperature    = meas.temperature.as_degrees_celsius();
            self.humidity       = meas.humidity.as_percent();
    // can remove service_needed? Don't need it?
            self.service_needed = false;
            self.sensor.sleep().unwrap();
        }
    }

    // maybe use this to convert to a suitable format for transmission? binary fixed-point?
    pub fn get_readings(&self) -> (f32, f32) {
        (self.temperature, self.humidity)
    }
}