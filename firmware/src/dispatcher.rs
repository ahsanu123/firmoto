use crate::controllers::spi_controller::SpiController;
use crate::firmoto_schema_generated::firmoto::{
    Operation, OperationType, SubOperationType, ValueT,
};
use alloc::vec::Vec;
use cortex_m_semihosting::hprintln;

pub struct Dispatcher;

impl Dispatcher {
    pub fn handle_operation(operation: Operation) {
        let op_name = operation.name().unwrap();
        hprintln!("handling {}", op_name);

        let op_ty = operation.op_type().0;
        let sub_op_ty = operation.sub_op_type().0;

        let op_args = operation
            .args()
            .unwrap()
            .iter()
            .map(|item| item.unpack())
            .collect::<Vec<ValueT>>();

        let _retval: Vec<u8> = match OperationType(op_ty) {
            OperationType::SPI => Self::handle_spi_op(SubOperationType(sub_op_ty), op_args),
            OperationType::I2C => Self::handle_i2c_op(SubOperationType(sub_op_ty), op_args),
            OperationType::GPIO => Self::handle_gpio_op(SubOperationType(sub_op_ty), op_args),
            _ => todo!(),
        };

        // NOTE:
        // build retval as binary
        // then send it with senderService, Usb or serial.
        // ||
        // SerialWriter::write(retval);
    }

    fn handle_spi_op(sub_op_ty: SubOperationType, args: Vec<ValueT>) -> Vec<u8> {
        let retval: Vec<u8> = match sub_op_ty {
            SubOperationType::SPI_WRITE_U8 => SpiController::write_u8(args),
            SubOperationType::SPI_READ_U8 => todo!(),
            SubOperationType::SPI_READ_U16 => todo!(),
            SubOperationType::SPI_READ_N => todo!(),
            _ => todo!(),
        };

        retval
    }

    fn handle_i2c_op(sub_op_ty: SubOperationType, args: Vec<ValueT>) -> Vec<u8> {
        todo!()
    }

    fn handle_gpio_op(sub_op_ty: SubOperationType, args: Vec<ValueT>) -> Vec<u8> {
        todo!()
    }
}
