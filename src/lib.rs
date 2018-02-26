#[macro_use]
extern crate serde_derive;
extern crate toml;

pub mod profile;
pub mod sensor;
pub mod config;

pub use profile::Profile;
pub use config::Configuration;

//pub fn run(profile: Profile, config: Configuration, )
