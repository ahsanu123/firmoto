use firmoto::writer::{SerialTransportErr, SerialTransportTrait};

pub struct Stm32SerialTransport;

impl SerialTransportTrait for Stm32SerialTransport {
    fn read(&mut self) -> alloc::vec::Vec<u8> {
        todo!()
    }

    fn write(&mut self, buffer: alloc::vec::Vec<u8>) -> Result<(), SerialTransportErr> {
        todo!()
    }
}
