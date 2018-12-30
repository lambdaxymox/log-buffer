extern crate log_buffer;

use log_buffer::LogBuffer;
use std::fmt::Write;


#[test]
fn test_empty_log_buffer_should_be_empty() {
    let log_buffer = LogBuffer::new([0x00; 16]);

    assert!(log_buffer.is_empty());
}

#[test]
fn test_empty_log_buffer_extract_should_be_empty_string() {
    let mut log_buffer = LogBuffer::new([0x00; 16]);
    let result = log_buffer.extract();
    let expected = "";

    assert_eq!(result, expected);

}

#[test]
fn test_empty_log_buffer_should_have_length_zero() {
    let mut log_buffer = LogBuffer::new([0x00; 16]);

    assert_eq!(log_buffer.len(), 0);
}

#[test]
fn test_log_buffer_should_have_capacity_equal_to_underlying_storage_size() {
    let mut storage = [0xFF as u8; 32];
    let mut log_buffer = LogBuffer::new(storage);

    assert_eq!(log_buffer.capacity(), storage.len());
}
