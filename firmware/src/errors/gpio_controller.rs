use crate::errors::get_err_msg_trait::GetErrMsgTrait;

pub enum GpioControllerErr {
    FailToChange,
    UnableToChangeIntoInput,
    UnableToChangeIntoOutput,
}

impl GetErrMsgTrait for GpioControllerErr {
    fn get_err_msg(self) -> alloc::string::String {
        match self {
            GpioControllerErr::FailToChange => "FailToChange".into(),
            GpioControllerErr::UnableToChangeIntoInput => "UnableToChangeIntoInput".into(),
            GpioControllerErr::UnableToChangeIntoOutput => "UnableToChangeIntoOutput".into(),
        }
    }
}
