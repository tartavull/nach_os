use crate::serial_println;
// TODO: serial_println only works during testing
// Modify Cargo.toml so that we can also debug from our host console.

const Green: &str = "\u{001b}[32m";
const White: &str = "\u{001b}[37m";

pub fn debug() {
    serial_println!("hello");
}

pub fn info() {
}

pub fn error() {
}

pub fn fatal() {
}
