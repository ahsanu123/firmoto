use crate::controllers::spi_controller::SpiControllerTrait;
use crate::firmoto_schema_generated::firmoto::{
    Operation, OperationType, SubOperationType, ValueT,
};
use crate::writer::SerialTransportTrait;
use alloc::vec::Vec;

pub struct Dispatcher<TSpi, TSerialTransport>
where
    TSpi: SpiControllerTrait,
    TSerialTransport: SerialTransportTrait,
{
    spi_controller: TSpi,
    serial_transport: TSerialTransport,
}

impl<TSpi, TSerialTransport> Dispatcher<TSpi, TSerialTransport>
where
    TSpi: SpiControllerTrait,
    TSerialTransport: SerialTransportTrait,
{
    //
    // let a_lot_of_controller = new a lot of controller
    // let dispatcher = Dispatcher::new(....a_lot_of_controller)
    // dispatcher.run()
    pub fn new(spi_controller: TSpi, serial_transport: TSerialTransport) -> Self {
        Self {
            spi_controller,
            serial_transport,
        }
    }

    pub fn run(&mut self) {
        let request = self.serial_transport.read();
        let operation = flatbuffers::root::<Operation>(&request).unwrap();
        self.handle_operation(operation);
    }

    pub fn handle_operation(&mut self, operation: Operation) {
        let _op_name = operation.name().unwrap();

        let op_ty = operation.op_type().0;
        let sub_op_ty = operation.sub_op_type().0;

        let op_args = operation
            .args()
            .unwrap()
            .iter()
            .map(|item| item.unpack())
            .collect::<Vec<ValueT>>();

        let retval: Vec<u8> = match OperationType(op_ty) {
            OperationType::SPI => self.handle_spi_op(SubOperationType(sub_op_ty), op_args),
            OperationType::I2C => self.handle_i2c_op(SubOperationType(sub_op_ty), op_args),
            OperationType::GPIO => self.handle_gpio_op(SubOperationType(sub_op_ty), op_args),
            _ => todo!(),
        };

        // TODO: do something with result
        let _ = self.serial_transport.write(retval);
    }

    fn handle_spi_op(&mut self, sub_op_ty: SubOperationType, args: Vec<ValueT>) -> Vec<u8> {
        let retval: Vec<u8> = match sub_op_ty {
            SubOperationType::SPI_WRITE_U8 => {
                self.spi_controller.write_u8(args);
                [0u8].to_vec()
            }
            SubOperationType::SPI_READ_U8 => todo!(),
            SubOperationType::SPI_READ_U16 => todo!(),
            SubOperationType::SPI_READ_N => todo!(),
            _ => todo!(),
        };

        retval
    }

    fn handle_i2c_op(&mut self, sub_op_ty: SubOperationType, args: Vec<ValueT>) -> Vec<u8> {
        todo!()
    }

    fn handle_gpio_op(&mut self, sub_op_ty: SubOperationType, args: Vec<ValueT>) -> Vec<u8> {
        todo!()
    }
}
