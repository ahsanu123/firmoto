use firmoto_playground::firmoto_schema_generated::firmoto::{
    Operation, OperationArgs, OperationType, Value, ValueArgs, ValueType,
};
use flatbuffers::FlatBufferBuilder;

fn main() {
    let mut builder = FlatBufferBuilder::with_capacity(2048);

    let op_name = builder.create_string("init_spi");

    let arg_sck = builder.create_string("sck");
    let arg_miso = builder.create_string("miso");
    let arg_mosi = builder.create_string("mosi");

    let arg1 = Value::create(
        &mut builder,
        &ValueArgs {
            name: Some(arg_sck),
            valtype: ValueType::U8,
            value: Some(arg_sck),
        },
    );

    let arg2 = Value::create(
        &mut builder,
        &ValueArgs {
            name: Some(arg_miso),
            valtype: ValueType::U8,
            value: Some(arg_miso),
        },
    );

    let arg3 = Value::create(
        &mut builder,
        &ValueArgs {
            name: Some(arg_mosi),
            valtype: ValueType::U8,
            value: Some(arg_mosi),
        },
    );

    let init_spi_args = builder.create_vector(&[arg1, arg2, arg3]);

    let init_spi = Operation::create(
        &mut builder,
        &OperationArgs {
            name: Some(op_name),
            optype: OperationType::SPI,
            args: Some(init_spi_args),
            reval: None,
        },
    );

    builder.finish(init_spi, None);

    let buf = builder.finished_data();

    let parsed_init_spi = flatbuffers::root::<Operation>(buf).unwrap();

    assert!(parsed_init_spi.name().is_some());
    let parsed_cmd_name = parsed_init_spi.name().unwrap();

    println!("command name: {parsed_cmd_name}");
}
