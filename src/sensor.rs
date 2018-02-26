pub trait Sensor {
    type Output;
    fn read(&self) -> Self::Output;
    fn temperatue(&self) -> u32;
    fn humidity(&self) -> u32;
}
