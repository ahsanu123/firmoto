use alloc::string::String;
use cortex_m_semihosting::hprintln;

pub enum Log {
    Info,
    Warning,
    Error,
}

pub fn log(ty: Log, message: &str) {
    let log_type_msg = match ty {
        Log::Info => String::from("[info]"),
        Log::Warning => String::from("[warning]"),
        Log::Error => String::from("[error]"),
    };
    let line = line!();
    let file = file!();

    // TODO: add another implementation
    hprintln!(
        "{} line: {} file: {} \n {}",
        log_type_msg,
        line,
        file,
        message
    );
}
