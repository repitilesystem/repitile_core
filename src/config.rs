//! Contains `Configuration` related functions and types.
//!
use toml;
use std::{self, fs::File, io::Read};

/// Contains information related to the configuration for the system
#[derive(Default, Serialize, Deserialize)]
pub struct Configuration {
    /// The time delay in seconds to wait before updating the sensors
    pub time_delay: u64,
    /// The default profile to load with this configuration
    pub default_profile: String,
}

impl Configuration {
    /// Loads the "default" default-config, `config.toml` in the current directory
    /// if it exists
    pub fn load_default() -> Result<Configuration, Box<std::error::Error>> {
        let mut f = File::open("config.toml")?;
        let mut contents = String::new();

        f.read_to_string(&mut contents)?;

        let config = toml::from_str(&contents)?;

        Ok(config)
    }

    /// Loads a specified configuration file
    pub fn load_file<P: AsRef<std::path::Path>>(
        p: P,
    ) -> Result<Configuration, Box<std::error::Error>> {
        let mut f = File::open(p)?;
        let mut contents = String::new();

        f.read_to_string(&mut contents)?;

        let config = toml::from_str(&contents)?;

        Ok(config)
    }
}
