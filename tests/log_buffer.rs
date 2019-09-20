extern crate log_buffer;

use log_buffer::LogBuffer;
use std::fmt::Write;


/// GIVEN: An log buffer.
/// WHEN: It reports being empty.
/// THEN: It should be empty.
#[test]
fn empty_log_buffer_should_be_empty() {
    let log_buffer = LogBuffer::new([0x00; 16]);

    assert!(log_buffer.is_empty());
}

/// GIVEN: A log buffer.
/// WHEN: The log buffer is empty.
/// THEN: `extract()` should return an empty string.
#[test]
fn empty_log_buffer_extract_should_be_empty_string() {
    let mut log_buffer = LogBuffer::new([0x00; 16]);

    let result = log_buffer.extract();
    let expected = "";

    assert_eq!(result, expected);

}

/// GIVEN: An empty log buffer.
/// WHEN: It reports the length in bytes.
/// THEN: The length should be zero.
#[test]
fn empty_log_buffer_should_have_length_zero() {
    let log_buffer = LogBuffer::new([0x00; 16]);

    assert_eq!(log_buffer.len(), 0);
}

/// GIVEN: A log buffer.
/// WHEN: It reports it capacity.
/// THEN: Its capacity should be equal to the size of the underlying storage.
#[test]
fn log_buffer_should_have_capacity_equal_to_underlying_storage_size() {
    let storage = [0xFF as u8; 32];
    let log_buffer = LogBuffer::new(storage);

    assert_eq!(log_buffer.capacity(), storage.len());
}

/// GIVEN: A log buffer and a string to insert of length <= buffer size.
/// WHEN: We call `extract()`.
/// THEN: `extract()` should return the exact same string.
#[test]
fn log_buffer_extracted_string_should_match_inserted_string_of_length_at_most_the_buffer_size() {
    let mut log_buffer = LogBuffer::new([0x00; 16]);
    write!(log_buffer, "abcdefghijklmnop").unwrap();

    let result = log_buffer.extract();
    let expected = "abcdefghijklmnop";

    assert_eq!(result, expected);
}

/// GIVEN: A filled log buffer.
/// WHEN: we call `is_full`.
/// THEN: It should return true.
#[test]
fn log_buffer_with_string_equal_to_length_in_bytes_should_be_full() {
    let mut log_buffer = LogBuffer::new([0x00; 16]);
    write!(log_buffer, "abcdefghijklmnop").unwrap();

    assert!(log_buffer.is_full());
}

/// GIVEN: A log buffer containing data.
/// WHEN: It reports whether it is empty.
/// THEN: It should not be empty.
#[test]
fn log_buffer_containing_data_should_not_be_empty() {
    let mut log_buffer = LogBuffer::new([0x00; 16]);
    write!(log_buffer, "abcdefghijklmnop").unwrap();

    assert!(!log_buffer.is_empty());
}

/// GIVEN: A log buffer containing data.
/// WHEN: We call ``.
/// THEN: Its length should be no larger than the capacity of the buffer.
#[test]
fn log_buffer_containing_data_should_have_length_no_larger_than_capacity() {
    let mut log_buffer = LogBuffer::new([0x00; 16]);
    write!(log_buffer, "abcdefghijklm").unwrap();

    assert!(log_buffer.len() <= log_buffer.capacity());
}

/// GIVEN: A log buffer and a string input longer than the buffer size.
/// WHEN: We write the string to the buffer.
/// THEN: The last buffer length worth of bytes in the string should be present.
#[test]
fn log_buffer_string_longer_than_buffer_length_should_wrap_around() {
    let mut log_buffer = LogBuffer::new([0x00; 16]);
    write!(log_buffer, "abcdefghijklmnopqrstuv").unwrap();

    let expected = "ghijklmnopqrstuv";
    let result = log_buffer.extract();

    assert_eq!(result, expected);
}

/// GIVEN: A log buffer
/// WHEN: We insert many rounds of data into it.
/// THEN: It should return the last buffer length worth of bytes written to it.
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

/// GIVEN: A log buffer and a string input longer than the buffer size.
/// WHEN: We write the string to the buffer.
/// THEN: The last buffer length worth of bytes in the string should be present.
#[test]
fn log_buffer_should_only_contain_the_last_buffer_length_number_of_bytes_put_into_it() {
    let mut log_buffer = LogBuffer::new([0x00; 16]);
    write!(log_buffer, "abcdefghijklmnopqrstuvwxyz1234567890").unwrap();

    let expected = "uvwxyz1234567890";
    let result = log_buffer.extract();

    assert_eq!(result, expected);
}

/// GIVEN: A log buffer with data.
/// WHEN: We clear the buffer.
/// THEN: The buffer should be empty.
#[test]
fn log_buffer_should_be_empty_after_clear() {
    let mut log_buffer = LogBuffer::new([0x00; 16]);
    write!(log_buffer, "abcdefghijklmnop").unwrap();
    log_buffer.clear();

    assert!(log_buffer.is_empty());
}

/// GIVEN: A log buffer containing data.
/// WHEN: We call `extract()` after clearing the buffer.
/// THEN: The string should be empty.
#[test]
fn log_buffer_extract_after_clear_should_be_empty_string() {
    let mut log_buffer = LogBuffer::new([0x00; 16]);
    write!(log_buffer, "abcdefghijklmnop").unwrap();
    log_buffer.clear();
    let result = log_buffer.extract();
    let expected = "";

    assert_eq!(result, expected);
}

/// GIVEN: A log buffer containing data.
/// WHEN: We call `extract()` multiple times in succession.
/// THEN: Each call to `extract()` should return the same string.
#[test]
fn log_buffer_extract_after_extract_should_yield_same_string() {
    let mut log_buffer = LogBuffer::new([0x00; 16]);
    write!(log_buffer, "abcdefghijklmn").unwrap();

    let result = String::from(log_buffer.extract());
    let expected = String::from(log_buffer.extract());

    assert_eq!(result, expected);
}

/// GIVEN: A filled log buffer.
/// WHEN: We call `space_remaining()`.
/// THEN: There should be no space remaining.
#[test]
fn filled_log_buffer_should_have_no_space_remaining() {
    let mut log_buffer = LogBuffer::new([0x00; 16]);
    write!(log_buffer, "abcdefghijklmnop").unwrap();

    assert_eq!(log_buffer.space_remaining(), 0);
}

/// GIVEN: An empty log buffer.
/// WHEN: We call `space_remaining()`.
/// THEN: The space remaining should be equal to the capacity of the buffer.
#[test]
fn empty_log_buffer_should_have_maximum_space_remaining() {
    let log_buffer = LogBuffer::new([0x00; 16]);

    assert_eq!(log_buffer.space_remaining(), log_buffer.capacity());
}
