extern crate repitile_core;
extern crate sysfs_gpio;

use sysfs_gpio::{Error, Pin};
use repitile_core::sensor::Sensor;

pub struct SimpleSensor {
    data_pin: Pin,
}

impl SimpleSensor {
    pub fn new(pin: u64) -> SimpleSensor {
        let pin = Pin::new(pin);
        pin.export();

        SimpleSensor { data_pin: pin }
    }
}

pub struct SimpleSensorOutput {
    pub is_active: bool,
}

impl Sensor for SimpleSensor {
    type Output = Result<SimpleSensorOutput, Error>;
    fn read(&self) -> Self::Output {
        let data = self.data_pin.get_value()?;

        Ok(SimpleSensorOutput {
            is_active: if data == 1 { true } else { false },
        })
    }

    fn temperatue(&self) -> u32 {
        85
    }

    fn humidity(&self) -> u32 {
        65
    }
}

impl Drop for SimpleSensor {
    fn drop(&mut self) {
        self.data_pin.unexport();
    }
}

fn main() {
    let sensor = SimpleSensor::new(16);
    let result = sensor.read().unwrap();

    println!("{:?}", result.is_active);
}
