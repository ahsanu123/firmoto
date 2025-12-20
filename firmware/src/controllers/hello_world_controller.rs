use alloc::format;

use crate::{
    errors::hello_world_controller_err::HelloWorldControllerErr,
    response_mappers::hello_world_response::HelloWorldRes,
};

type CRes<T> = Result<T, HelloWorldControllerErr>;

pub trait HelloWorlControllerTrait {
    fn add(&mut self, num_a: u8, num_b: u8) -> CRes<HelloWorldRes>;
    fn add_but_error(&mut self) -> CRes<HelloWorldRes>;
}

#[derive(Default)]
pub struct HelloWorlController;

impl HelloWorlControllerTrait for HelloWorlController {
    fn add(&mut self, num_a: u8, num_b: u8) -> CRes<HelloWorldRes> {
        let result = num_a + num_b;
        let response = HelloWorldRes {
            status: format!("{} plus {} is {}", num_a, num_b, result),
            value: result,
        };

        Ok(response)
    }

    fn add_but_error(&mut self) -> CRes<HelloWorldRes> {
        // directly return error
        Err(HelloWorldControllerErr::AnError("this is an error".into()))
    }
}
