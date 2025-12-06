#![no_main]
#![no_std]

extern crate alloc;

use core::mem::MaybeUninit;
use cortex_m_semihosting::hprintln;
use embedded_alloc::LlffHeap as Heap;
use flatbuffers::FlatBufferBuilder;
use nostd_playground::{
    self as _,
    firmoto_schema_generated::firmoto::{
        Operation, OperationArgs, OperationType, SubOperationType, Value, ValueArgs, ValueType,
    },
};

#[global_allocator]
static HEAP: Heap = Heap::empty();

#[cortex_m_rt::entry]
fn main() -> ! {
    const HEAP_SIZE: usize = 4096;
    static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
    unsafe {
        HEAP.init(&raw mut HEAP_MEM as usize, HEAP_SIZE);
    }

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

    let buf = builder.finished_data();

    let parsed_init_spi = flatbuffers::root::<Operation>(buf).unwrap();

    assert!(parsed_init_spi.name().is_some());
    let parsed_cmd_name = parsed_init_spi.name().unwrap();

    hprintln!("command name: {}", parsed_cmd_name);

    let parsed_args = parsed_init_spi.args().unwrap();

    parsed_args.iter().for_each(|item| {
        let args_name = item.name().unwrap();
        let args_type = item.valtype().0;
        let args_value = item.value().unwrap();

        hprintln!("\t - args name:{}", args_name);
        hprintln!("\t \t * type: {}", args_type);
        hprintln!("\t \t * type: {}", args_value);
    });

    nostd_playground::exit()
}
