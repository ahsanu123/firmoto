use alloc::vec::Vec;

pub enum SerialTransportErr {
    NotSend,
    // etc etc
}

pub trait SerialTransportTrait {
    fn read(&mut self) -> Vec<u8>;
    fn write(&mut self, buffer: Vec<u8>) -> Result<(), SerialTransportErr>;
}
