extern crate log_buffer;

use log_buffer::LogBuffer;
use std::fmt::Write;


fn main() {
    let storage = [0xFF as u8; 32];
    let mut log_buffer = LogBuffer::new(storage);

    assert!(log_buffer.is_empty());

    writeln!(log_buffer, "A string.").unwrap();
    writeln!(log_buffer, "Another string.").unwrap();

    assert!(!log_buffer.is_empty());
    assert_eq!(log_buffer.extract(), "A string.\nAnother string.\n");

    println!("{}", log_buffer.extract());
}
