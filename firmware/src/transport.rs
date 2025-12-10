use alloc::vec::Vec;

use crate::errors::transport::SerialTransportErr;

pub trait SerialTransportTrait {
    fn read(&mut self) -> Vec<u8>;
    fn write(&mut self, buffer: Vec<u8>) -> Result<(), SerialTransportErr>;
}
