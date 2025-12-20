use alloc::string::String;

use crate::errors::get_err_msg_trait::GetErrMsgTrait;

pub enum HelloWorldControllerErr {
    AnError(String),
}

impl GetErrMsgTrait for HelloWorldControllerErr {
    fn get_err_msg(self) -> String {
        match self {
            HelloWorldControllerErr::AnError(err) => err,
        }
    }
}
