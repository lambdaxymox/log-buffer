extern crate log_buffer;

use log_buffer::LogBuffer;
use std::fmt::Write;


#[test]
fn test_empty_log_buffer_should_be_empty() {
    let log_buffer = LogBuffer::new([0x00; 16]);

    assert!(log_buffer.is_empty());
}

#[test]
fn test_empty_log_buffer_should_have_no_text() {
    let mut log_buffer = LogBuffer::new([0x00; 16]);
    let result = log_buffer.extract();
    let expected = "";

    assert_eq!(result, expected);

}

