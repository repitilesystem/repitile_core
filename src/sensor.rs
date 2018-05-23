//! Sensor trait.

/// Describes a usable sensor. `read` will be called upon the timer
/// ending and triggering a sensor data update. `temperature` and
/// `humidity` both get their respective values which the system will use
/// to make decisions on whether systems should be on or off.
pub trait Sensor {
    /// Called periodically to have the sensor struct update its information.
    fn read(&mut self);
    /// Returns the temperature read from the sensor.
    fn temperature(&self) -> u32;
    /// Returns the humidity read from the sensor.
    fn humidity(&self) -> u32;
}
