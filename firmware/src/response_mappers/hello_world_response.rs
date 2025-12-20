use alloc::{string::String, vec::Vec};
use flatbuffers::FlatBufferBuilder;

use crate::{
    errors::hello_world_controller_err::HelloWorldControllerErr,
    schema_generated::firmoto::{
        FieldString, FieldStringArgs, FieldType, FieldU8, FieldU8Args, ReturnValue,
        ReturnValueArgs, Value, ValueArgs,
    },
};

pub trait ToResponseTrait<T, E> {
    fn to_response(&mut self, result: Result<T, E>) -> Vec<u8>;
}

pub struct HelloWorldRes {
    pub status: String,
    pub value: u8,
}

impl ToResponseTrait<HelloWorldRes, HelloWorldControllerErr> for HelloWorldRes {
    fn to_response(&mut self, result: Result<HelloWorldRes, HelloWorldControllerErr>) -> Vec<u8> {
        let mut builder = FlatBufferBuilder::with_capacity(2048);

        match result {
            Ok(result) => {
                // TODO:
                // convert this to macro
                // ======================================================
                let status_builded = builder.create_string(&result.status);
                let status_val = FieldString::create(
                    &mut builder,
                    &FieldStringArgs {
                        field: Some(status_builded),
                    },
                );

                let status_name = builder.create_string("status");
                let status_value = Value::create(
                    &mut builder,
                    &ValueArgs {
                        name: Some(status_name),
                        value_type: FieldType::FieldString,
                        value: Some(status_val.as_union_value()),
                    },
                );

                // ======================================================
                let value_val = FieldU8::create(
                    &mut builder,
                    &FieldU8Args {
                        field: result.value,
                    },
                );

                let value_name = builder.create_string("value");
                let value_value = Value::create(
                    &mut builder,
                    &ValueArgs {
                        name: Some(value_name),
                        value_type: FieldType::FieldU8,
                        value: Some(value_val.as_union_value()),
                    },
                );
                // ======================================================

                let arr_ret_val = builder.create_vector(&[status_value, value_value]);

                let returnval = ReturnValue::create(
                    &mut builder,
                    &ReturnValueArgs {
                        data: Some(arr_ret_val),
                    },
                );
                builder.finish(returnval, None);
                builder.finished_data().to_vec()
            }
            Err(err) => {
                let HelloWorldControllerErr::AnError(err_msg) = err;

                let error_ms_builded = builder.create_string(err_msg.as_str());
                let error_val = FieldString::create(
                    &mut builder,
                    &FieldStringArgs {
                        field: Some(error_ms_builded),
                    },
                );

                let error_name = builder.create_string("error");
                let error_value = Value::create(
                    &mut builder,
                    &ValueArgs {
                        name: Some(error_name),
                        value_type: FieldType::FieldString,
                        value: Some(error_val.as_union_value()),
                    },
                );
                let arr_ret_val = builder.create_vector(&[error_value]);

                let returnval = ReturnValue::create(
                    &mut builder,
                    &ReturnValueArgs {
                        data: Some(arr_ret_val),
                    },
                );
                builder.finish(returnval, None);
                builder.finished_data().to_vec()
            }
        }
    }
}
