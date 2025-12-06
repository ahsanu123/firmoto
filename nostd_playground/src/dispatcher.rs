use alloc::vec::Vec;
use cortex_m_semihosting::hprintln;

use crate::firmoto_schema_generated::firmoto::{
    Operation, OperationType, SubOperationType, Value, ValueT,
};
pub struct Dispatcher;

impl Dispatcher {
    pub fn handle_operation(operation: Operation) {
        let op_name = operation.name().unwrap();
        hprintln!("handling {op_name}");

        let op_ty = operation.op_type().0;
        let sub_op_ty = operation.sub_op_type().0;

        let op_args = operation
            .args()
            .unwrap()
            .iter()
            .map(|item| item.unpack())
            .collect::<Vec<ValueT>>();

        match OperationType(op_ty) {
            OperationType::SPI => Self::handle_spi_op(SubOperationType(sub_op_ty), op_args),
            OperationType::I2C => Self::handle_i2c_op(SubOperationType(sub_op_ty), op_args),
            OperationType::GPIO => Self::handle_gpio_op(SubOperationType(sub_op_ty), op_args),
            _ => {}
        };
    }

    fn handle_spi_op(sub_op_ty: SubOperationType, args: Vec<ValueT>) {
        // NOTE:
        // this should be return Result<Retval>
        // no matter it success or fail,
        // retval will handled by client (success, or fail)

        match sub_op_ty {
            SubOperationType::SPI_WRITE_U8 => {
                // NOTE: call controller like
                // spi_controller_instance.write_u8(args)
                // dont do parsing args here, this part
                // is only "sender", parsing should done in
                // controller
                todo!()
            }
            SubOperationType::SPI_READ_U8 => todo!(),
            SubOperationType::SPI_READ_U16 => todo!(),
            SubOperationType::SPI_READ_N => todo!(),
            _ => {}
        }

        // NOTE:
        // build retval as binary
        // then send it with senderService, Usb or serial.
    }

    fn handle_i2c_op(sub_op_ty: SubOperationType, args: Vec<ValueT>) {
        todo!()
    }

    fn handle_gpio_op(sub_op_ty: SubOperationType, args: Vec<ValueT>) {
        todo!()
    }
}
