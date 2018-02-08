use serde_derive;
use toml;
use std::io::{Read, Write};//, Error, ErrorKind};
use std::fs::File;

#[derive(Debug)]
pub enum ProfileError {
    DeserializationError,
    SerializationError,
    LoadError,
    SaveError,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    name: String,
    temps: Temps,
    humidity: Humidity,
    light: Light
}

impl Default for Profile {
    fn default() -> Profile {
        Profile {
            name: String::from("Default Reptile"),
            temps: Temps::default(),
            humidity: Humidity::default(),
            light: Light::default()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Temps {
    max: i32,
    min: i32,
}

impl Default for Temps {
    fn default() -> Temps {
        Temps {
            max: 85,
            min: 75
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Humidity {
    max: i32,
    min: i32,
}

impl Default for Humidity {
    fn default() -> Humidity {
        Humidity {
            max: 85,
            min: 75
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Light {
    on: toml::value::Datetime,
    off: toml::value::Datetime,
}

impl Default for Light {
    fn default() -> Light {
        let light_str = "on = 00:00:00\noff = 12:00:00";
        toml::from_str(light_str).unwrap()
    }
}

impl Profile {
    pub fn new() -> Profile {
        Profile::default()
    }

    pub fn temp_range(&self) -> (i32, i32) {
        (self.temps.min, self.temps.max)
    }

    pub fn humidity_range(&self) -> (i32, i32) {
        (self.humidity.min, self.humidity.max)
    }

    pub fn save_to_file<T: AsRef<::std::path::Path>>(&self, filename: T) -> Result<(), ProfileError> {
        let serialized = toml::to_string(&self).map_err(|_| ProfileError::SerializationError)?;
        let mut save_file = File::create(filename).map_err(|_| ProfileError::SaveError)?;

        save_file.write_all(serialized.as_bytes()).map_err(|_| ProfileError::SaveError)?;

        Ok(())
    }

    pub fn read_from_file<T: AsRef<::std::path::Path>>(filename: T) -> Result<Profile, ProfileError> {
        let mut contents = String::new();

        let mut read_file = File::open(filename).map_err(|_| ProfileError::LoadError)?;

        read_file.read_to_string(&mut contents).map_err(|_| ProfileError::SaveError)?;
        
        let deserialized = toml::from_str::<Profile>(&contents).map_err(|_| ProfileError::DeserializationError)?;
        
        Ok(deserialized)
    }
}