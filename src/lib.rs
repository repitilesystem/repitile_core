//! `repitile_core` contains the core functionality for monitoring
//! and regulating the environment of the reptile tank.
//!
//! See individual struct/module documentation for better explanations of each part.

#![deny(missing_docs)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate time;
extern crate timer;
extern crate toml;

pub mod profile;
pub mod sensor;
pub mod config;
pub mod regulator;

use profile::Profile;
use config::Configuration;
use sensor::Sensor;
use regulator::Regulator;

use chrono::{DateTime, Local};

use std::sync::{Arc, Mutex};
use std::sync::mpsc::{self, Receiver, SyncSender};

use time::Duration;

/// Describes messages passed between the core and server.
#[derive(Debug)]
pub enum CommReq {
    /// Returns the current temperature.
    GetTemp,
    /// Returns the current humidity.
    GetHumid,
    /// Request to load a new profile.
    OpenProfile(String),
    /// Request to load a new config.
    OpenConfig(String),
    /// Response to a `GetTemp` request.
    SendTemp(u32),
    /// Response to a `GetHumid` request.
    SendHumid(u32),
    /// Successfully executed request.
    Ok,
    /// Failed to execute request.
    Err,
}

/// Describes the current conditions of the environment.
#[derive(Debug)]
pub struct CurrentConditions {
    /// Whether or not the light is currently on.
    pub light: bool,
    /// The average temperature.
    pub temp: u32,
    /// The average humidity.
    pub humidity: u32,
    /// The time at which the struct was last updated.
    pub time: DateTime<Local>,
}

lazy_static! {
    /// Global current conditions that can be accessed by other threads.
    pub static ref CURRENT_CONDITIONS: Mutex<CurrentConditions> = Mutex::new(CurrentConditions {
        light: false,
        temp: 0,
        humidity: 0,
        time: Local::now(),
    });
}

/// The core structure that monitors the environment.
#[allow(dead_code)]
pub struct Core {
    /// The current configuration.
    config: Configuration,
    /// The current profile.
    profile: Profile,
    /// A thread-safe collection of sensors that will be called to update conditions.
    sensors: Arc<Mutex<Vec<Box<Sensor + Send>>>>,
    /// A thread-safe collection of regulators that will be called to see if the environment
    /// needs adjusted.
    regulators: Arc<Mutex<Vec<Box<Regulator + Send>>>>,
}

impl Core {
    /// Creates a new `Core`, with no sensors or regulators set.
    pub fn new(c: Configuration, p: Profile) -> Core {
        Core {
            config: c,
            profile: p,
            sensors: Arc::new(Mutex::new(Vec::new())),
            regulators: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Returns a reference to the current profile.
    pub fn profile(&self) -> &Profile {
        &self.profile
    }

    /// Allows the user to add sensors that `impl Sensor`.
    pub fn add_sensor<S: Sensor + 'static + Send>(&mut self, s: S) {
        (*self.sensors.lock().unwrap()).push(Box::new(s));
    }

    /// Allows the user to add regulators that `impl Regulator`.
    pub fn add_regulator<R: Regulator + 'static + Send>(&mut self, s: R) {
        (*self.regulators.lock().unwrap()).push(Box::new(s));
    }

    /// Begins monitoring of the environment. Updates sensors and regulators
    /// every X seconds, determined by the configuration value. Listens for
    /// and responds to communication requests using two sync channels.
    pub fn run(&mut self, send: SyncSender<CommReq>, recv: Receiver<CommReq>) {
        let sensor_timer = timer::Timer::new();
        let time_timer = timer::Timer::new();
        let sensor_clone = self.sensors.clone();
        let regulator_clone = self.regulators.clone();

        let _time_guard = time_timer.schedule_with_delay(Duration::seconds(1), || {
            CURRENT_CONDITIONS.lock().unwrap().time = Local::now();
        });

        let _guard = sensor_timer.schedule_repeating(
            Duration::seconds(self.config.time_delay as i64),
            move || {
                println!("Updating sensors @ {}", Local::now());
                let mut sensors = sensor_clone.lock().unwrap();
                let mut temp = 0;
                let mut humid = 0;

                for (i, sensor) in (*sensors).iter_mut().enumerate() {
                    sensor.read();

                    temp += sensor.temperature();
                    humid += sensor.humidity();

                    println!(
                        "Sensor {} read:\n    --> temp: {} deg C\n    --> humd: {}%",
                        i,
                        sensor.temperature(),
                        sensor.humidity()
                    );
                }

                temp /= (*sensors).len() as u32;
                humid /= (*sensors).len() as u32;

                let mut cur_cond = CURRENT_CONDITIONS.lock().unwrap();

                cur_cond.temp = temp;
                cur_cond.humidity = humid;

                println!("Updating regulators @ {}", Local::now());

                let mut regs = regulator_clone.lock().unwrap();

                for (i, reg) in (*regs).iter_mut().enumerate() {
                    println!("Updating regulator {}", i);
                    reg.update(&cur_cond);
                }
            },
        );

        loop {
            match recv.try_recv() {
                Ok(cmd) => match cmd {
                    CommReq::GetTemp => {
                        send.send(CommReq::SendTemp(5)).unwrap();
                    }
                    CommReq::GetHumid => {
                        send.send(CommReq::SendHumid(6)).unwrap();
                    }
                    CommReq::OpenProfile(prof) => {
                        if let Ok(prof) = Profile::read_from_file(prof) {
                            self.profile = prof;

                            let mut reg_lock = self.regulators.lock().unwrap();

                            for reg in (*reg_lock).iter_mut() {
                                reg.profile_changed(&self.profile);
                            }

                            send.send(CommReq::Ok).unwrap();
                        } else {
                            send.send(CommReq::Err).unwrap();
                        }
                    }
                    CommReq::OpenConfig(prof) => {
                        if let Ok(config) = Configuration::load_file(prof) {
                            if let Ok(prof) = Profile::read_from_file(&config.default_profile) {
                                self.config = config;
                                self.profile = prof;

                                let mut reg_lock = self.regulators.lock().unwrap();

                                for reg in (*reg_lock).iter_mut() {
                                    reg.profile_changed(&self.profile);
                                }

                                send.send(CommReq::Ok).unwrap();
                            } else {
                                send.send(CommReq::Err).unwrap();
                            }
                        } else {
                            send.send(CommReq::Err).unwrap();
                        }
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
