#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
extern crate time;
extern crate timer;
extern crate toml;

pub mod profile;
pub mod sensor;
pub mod config;

use profile::Profile;
use config::Configuration;
use sensor::Sensor;

use std::sync::{Arc, Mutex};
use std::sync::mpsc::{self, Receiver, SyncSender};

use time::Duration;

#[derive(Debug)]
pub enum CommReq {
    GetTemp,
    GetHumid,
    OpenProfile(String),
    OpenConfig(String),
    SendTemp(u32),
    SendHumid(u32),
}

#[derive(Debug)]
pub struct CurrentConditions {
    pub light: bool,
    pub temp: u32,
    pub humidity: u32,
}

lazy_static! {
    pub static ref CURRENT_CONDITIONS: Mutex<CurrentConditions> = Mutex::new(CurrentConditions {
        light: false,
        temp: 0,
        humidity: 0,
    });
}

pub struct Core {
    config: Configuration,
    profile: Profile,
    sensors: Arc<Mutex<Vec<Box<Sensor + Send>>>>,
}

impl Core {
    pub fn new(c: Configuration, p: Profile) -> Core {
        Core {
            config: c,
            profile: p,
            sensors: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn add_sensor<S: Sensor + 'static + Send>(&mut self, s: S) {
        (*self.sensors.lock().unwrap()).push(Box::new(s));
    }

    pub fn run(&mut self, send: SyncSender<CommReq>, recv: Receiver<CommReq>) {
        let sensor_timer = timer::Timer::new();
        let sensor_clone = self.sensors.clone();
        let _guard = sensor_timer.schedule_repeating(
            Duration::seconds(self.config.time_delay as i64),
            move || {
                let mut sensors = sensor_clone.lock().unwrap();
                for (i, sensor) in (*sensors).iter_mut().enumerate() {
                    sensor.read();
                    println!("Sensor {} read: {}", i, sensor.temperature());
                }
            },
        );

        loop {
            match recv.try_recv() {
                Ok(cmd) => match cmd {
                    CommReq::GetTemp => {
                        send.send(CommReq::SendTemp(5));
                    }
                    CommReq::GetHumid => {
                        send.send(CommReq::SendHumid(6));
                    }
                    _ => {}
                },
                Err(e) => match e {
                    mpsc::TryRecvError::Disconnected => break,
                    mpsc::TryRecvError::Empty => {}
                },
            };
        }
    }
}
