use firmoto_playground::firmoto_schema_generated::firmoto::{
    Operation, OperationArgs, OperationType, SubOperationType, Value, ValueArgs, ValueType,
};
use flatbuffers::FlatBufferBuilder;
use std::fs::{self, File};
use std::io::Write;

fn save_binary(buf: &[u8], path: &str) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(buf)?;
    Ok(())
}

fn read_binary(path: &str) -> std::io::Result<Vec<u8>> {
    let data = fs::read(path)?;
    Ok(data)
}

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
            op_type: OperationType::SPI,
            sub_op_type: SubOperationType::SPI_READ_U8,
            args: Some(init_spi_args),
            retval: None,
        },
    );

    builder.finish(init_spi, None);

    // ========================================================
    // Parser Side
    // ========================================================
    let buf = builder.finished_data();
    let _ = save_binary(buf, "./init_spi.bin");

    let binary = read_binary("./init_spi.bin").unwrap();

    // from buf directly
    // let parsed_init_spi = flatbuffers::root::<Operation>(buf).unwrap();

    // from binary data
    let parsed_init_spi = flatbuffers::root::<Operation>(&binary).unwrap();

    assert!(parsed_init_spi.name().is_some());
    let parsed_cmd_name = parsed_init_spi.name().unwrap();

    println!("command name: {parsed_cmd_name}");

    assert!(parsed_init_spi.args().is_some());
    let parsed_args = parsed_init_spi.args().unwrap();

    parsed_args.iter().for_each(|item| {
        let args_name = item.name().unwrap();
        let args_type = item.valtype().0;
        let args_value = item.value().unwrap();

        println!("\t - args name:{args_name} ");
        println!("\t \t * type: {args_type:?}");
        println!("\t \t * type: {args_value}");
    });
}
