extern crate log_buffer;

use log_buffer::LogBuffer;
use std::fmt::Write;


fn main() {
    let mut storage = vec![0xFF as u8; 128];
    let mut log_buffer = LogBuffer::new(storage);

    assert!(log_buffer.is_empty());

    writeln!(log_buffer, "This is a string.").unwrap();
    writeln!(log_buffer, "This is another string.").unwrap();
    writeln!(log_buffer, "This is yet another string.").unwrap();

    assert!(!log_buffer.is_empty());
    assert_eq!(
        log_buffer.extract(),
        "This is a string.\nThis is another string.\nThis is yet another string.\n"
    );

    println!("{}", log_buffer.extract());
}
