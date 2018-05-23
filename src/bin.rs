extern crate repitile_core;
extern crate sysfs_gpio;

use sysfs_gpio::Pin;
use repitile_core::sensor::Sensor;

pub struct SimpleSensor {
    data_pin: Pin,
    pub is_active: bool,
}

impl SimpleSensor {
    pub fn new(pin: u64) -> SimpleSensor {
        let pin = Pin::new(pin);
        pin.export().unwrap();

        SimpleSensor {
            data_pin: pin,
            is_active: false,
        }
    }
}

impl Sensor for SimpleSensor {
    fn read(&mut self) {
        let data = self.data_pin.get_value().unwrap();

        self.is_active = if data == 1 { true } else { false };
    }

    fn temperature(&self) -> u32 {
        85
    }

    fn humidity(&self) -> u32 {
        65
    }
}

impl Drop for SimpleSensor {
    fn drop(&mut self) {
        self.data_pin.unexport().unwrap();
    }
}

fn main() {
    let mut sensor = SimpleSensor::new(16);
    sensor.read();

    println!("{:?}", sensor.is_active);
}
