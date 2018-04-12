//! Contains `Profile` related functions and types.

use toml;
use std::io::{Read, Write}; //, Error, ErrorKind};
use std::fs::File;

/// Profile error possibilities
#[derive(Debug)]
pub enum ProfileError {
    /// TOML Deserialization failed.
    DeserializationError,
    /// TOML Serialization failed.
    SerializationError,
    /// Failed to load the profile file.
    LoadError,
    /// Failed to save the profile file.
    SaveError,
}

/// `Profile` contains all the relevant information for a reptile environment.
#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    /// Profile name
    name: String,
    /// Min and max temperatures.
    temps: Temps,
    /// Min and max humidity.
    humidity: Humidity,
    /// On and off times for any lights.
    light: Light,
}

impl Default for Profile {
    fn default() -> Profile {
        Profile {
            name: String::from("Default Reptile"),
            temps: Temps::default(),
            humidity: Humidity::default(),
            light: Light::default(),
        }
    }
}

/// Describes the range for acceptable temperatures.
#[derive(Debug, Serialize, Deserialize)]
pub struct Temps {
    /// Max temperature.
    pub max: i32,
    /// Min temperature.
    pub min: i32,
}

impl Default for Temps {
    fn default() -> Temps {
        Temps { max: 30, min: 25 }
    }
}

/// Describes the range for acceptable humidity.
#[derive(Debug, Serialize, Deserialize)]
pub struct Humidity {
    /// Max humidity.
    pub max: i32,
    /// Min humidity.
    pub min: i32,
}

impl Default for Humidity {
    fn default() -> Humidity {
        Humidity { max: 100, min: 30 }
    }
}

/// Contains the times when the light should be on and off.
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
    /// Creates a new, default profile.
    pub fn new() -> Profile {
        Profile::default()
    }

    /// Returns the profile temperature range.
    pub fn temp_range(&self) -> &Temps {
        &self.temps
    }

    /// Returns the profile humidity range.
    pub fn humidity_range(&self) -> &Humidity {
        &self.humidity
    }

    /// Save the current profile to the specified file.
    pub fn save_to_file<T: AsRef<::std::path::Path>>(
        &self,
        filename: T,
    ) -> Result<(), ProfileError> {
        let serialized = toml::to_string(&self).map_err(|_| ProfileError::SerializationError)?;
        let mut save_file = File::create(filename).map_err(|_| ProfileError::SaveError)?;

        save_file
            .write_all(serialized.as_bytes())
            .map_err(|_| ProfileError::SaveError)?;

        Ok(())
    }

    /// Load a profile from a specified file.
    pub fn read_from_file<T: AsRef<::std::path::Path>>(
        filename: T,
    ) -> Result<Profile, ProfileError> {
        let mut contents = String::new();

        let mut read_file = File::open(filename).map_err(|_| ProfileError::LoadError)?;

        read_file
            .read_to_string(&mut contents)
            .map_err(|_| ProfileError::SaveError)?;

        let deserialized =
            toml::from_str::<Profile>(&contents).map_err(|_| ProfileError::DeserializationError)?;

        Ok(deserialized)
    }
}
