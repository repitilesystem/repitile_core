/// Describes a usable sensor. `read` will be called upon the timer
/// ending and triggering a sensor data update. `temperature` and
/// `humidity` both get their respective values which the system will use
/// to make decisions on whether systems should be on or off.
pub trait Sensor {
    fn read(&mut self);
    fn temperature(&self) -> u32;
    fn humidity(&self) -> u32;
}
