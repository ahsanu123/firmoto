use crate::{
    schema_generated::firmoto::{
        ReturnValue, ReturnValueArgs, Value, ValueArgs, ValueT, ValueType,
    },
    service_wrappers::SpiServiceWrapperTrait,
};
use alloc::vec::Vec;
use flatbuffers::FlatBufferBuilder;

pub trait SpiControllerTrait {
    fn write_u8(&mut self, args: Vec<ValueT>) -> Vec<u8>;
}

pub struct SpiController<T>
where
    T: SpiServiceWrapperTrait,
{
    wrapper: T,
}

impl<T> SpiController<T>
where
    T: SpiServiceWrapperTrait,
{
    pub fn new(spi_wrapper: T) -> Self {
        Self {
            wrapper: spi_wrapper,
        }
    }
}

impl<T> SpiControllerTrait for SpiController<T>
where
    T: SpiServiceWrapperTrait,
{
    fn write_u8(&mut self, _args: Vec<ValueT>) -> Vec<u8> {
        let mut builder = FlatBufferBuilder::with_capacity(2048);

        let result = self.wrapper.write_u8(8u8, 8u8);

        match result {
            Ok(_) => {
                let success_msg = builder.create_string("success");
                let succes = Value::create(
                    &mut builder,
                    &ValueArgs {
                        name: Some(success_msg),
                        valtype: ValueType::U16,
                        value: Some(success_msg),
                    },
                );

                let success2 = Value::create(
                    &mut builder,
                    &ValueArgs {
                        name: Some(success_msg),
                        valtype: ValueType::U16,
                        value: Some(success_msg),
                    },
                );

                let success_arr = builder.create_vector(&[succes, success2]);
                let succes_retval = ReturnValue::create(
                    &mut builder,
                    &ReturnValueArgs {
                        data: Some(success_arr),
                    },
                );

                builder.finish(succes_retval, None);
                let buf = builder.finished_data();

                buf.to_vec()
            }
            Err(_) => {
                let error_msg = builder.create_string("error_message");
                let error = Value::create(
                    &mut builder,
                    &ValueArgs {
                        name: Some(error_msg),
                        valtype: ValueType::U16,
                        value: Some(error_msg),
                    },
                );

                let error1 = Value::create(
                    &mut builder,
                    &ValueArgs {
                        name: Some(error_msg),
                        valtype: ValueType::U16,
                        value: Some(error_msg),
                    },
                );

                let error_arr = builder.create_vector(&[error, error1]);
                let error_retval = ReturnValue::create(
                    &mut builder,
                    &ReturnValueArgs {
                        data: Some(error_arr),
                    },
                );

                builder.finish(error_retval, None);
                let buf = builder.finished_data();

                buf.to_vec()
            }
        }
    }
}
