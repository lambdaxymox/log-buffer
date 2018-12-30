extern crate log_buffer;

use log_buffer::LogBuffer;
use std::fmt::Write;


///
///
#[test]
fn empty_log_buffer_should_be_empty() {
    let log_buffer = LogBuffer::new([0x00; 16]);

    assert!(log_buffer.is_empty());
}

#[test]
fn empty_log_buffer_extract_should_be_empty_string() {
    let mut log_buffer = LogBuffer::new([0x00; 16]);

    let result = log_buffer.extract();
    let expected = "";

    assert_eq!(result, expected);

}

#[test]
fn empty_log_buffer_should_have_length_zero() {
    let log_buffer = LogBuffer::new([0x00; 16]);

    assert_eq!(log_buffer.len(), 0);
}

#[test]
fn log_buffer_should_have_capacity_equal_to_underlying_storage_size() {
    let storage = [0xFF as u8; 32];
    let log_buffer = LogBuffer::new(storage);

    assert_eq!(log_buffer.capacity(), storage.len());
}

#[test]
fn log_buffer_extracted_string_should_match_inserted_string_of_length_at_most_the_buffer_size() {
    let mut log_buffer = LogBuffer::new([0x00; 16]);
    write!(log_buffer, "abcdefghijklmnop").unwrap();

    let result = log_buffer.extract();
    let expected = "abcdefghijklmnop";

    assert_eq!(result, expected);
}

#[test]
fn log_buffer_with_string_equal_to_length_in_bytes_should_be_full() {
    let mut log_buffer = LogBuffer::new([0x00; 16]);
    write!(log_buffer, "abcdefghijklmnop").unwrap();

    assert!(log_buffer.is_full());
}

#[test]
fn log_buffer_containing_data_should_not_be_empty() {
    let mut log_buffer = LogBuffer::new([0x00; 16]);
    write!(log_buffer, "abcdefghijklmnop").unwrap();

    assert!(!log_buffer.is_empty());
}

#[test]
fn log_buffer_string_longer_than_buffer_length_should_wrap_around() {
    let mut log_buffer = LogBuffer::new([0x00; 16]);
    write!(log_buffer, "abcdefghijklmnopqrstuv").unwrap();

    let expected = "ghijklmnopqrstuv";
    let result = log_buffer.extract();

    assert_eq!(result, expected);
}

#[test]
fn log_buffer_should_correctly_extract_data_after_multiple_cycles() {
    let mut log_buffer = LogBuffer::new([0x00; 16]);
    write!(log_buffer, "abcdefghijklmnop").unwrap();
    write!(log_buffer, "abcdefghijklmnop").unwrap();
    write!(log_buffer, "abcdefghijklmnop").unwrap();
    write!(log_buffer, "abcdefghijklmnopqrstuv").unwrap();

    let expected = "ghijklmnopqrstuv";
    let result = log_buffer.extract();

    assert_eq!(result, expected);
}

#[test]
fn log_buffer_should_only_contain_the_last_buffer_length_number_of_bytes_put_into_it() {
    let mut log_buffer = LogBuffer::new([0x00; 16]);
    write!(log_buffer, "abcdefghijklmnopqrstuvwxyz1234567890").unwrap();

    let expected = "uvwxyz1234567890";
    let result = log_buffer.extract();

    assert_eq!(result, expected);
}

#[test]
fn log_buffer_should_be_empty_after_clear() {
    let mut log_buffer = LogBuffer::new([0x00; 16]);
    write!(log_buffer, "abcdefghijklmnop").unwrap();
    log_buffer.clear();

    assert!(log_buffer.is_empty());
}

#[test]
fn log_buffer_extract_after_clear_should_be_empty_string() {
    let mut log_buffer = LogBuffer::new([0x00; 16]);
    write!(log_buffer, "abcdefghijklmnop").unwrap();
    log_buffer.clear();
    let result = log_buffer.extract();
    let expected = "";

    assert_eq!(result, expected);
}

#[test]
fn log_buffer_extract_after_extract_should_yield_same_string() {
    let mut log_buffer = LogBuffer::new([0x00; 16]);
    write!(log_buffer, "abcdefghijklmn").unwrap();

    let result = String::from(log_buffer.extract());
    let expected = String::from(log_buffer.extract());

    assert_eq!(result, expected);
}

