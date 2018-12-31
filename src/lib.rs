//!
//! The `log_buffer` create provides a mean of storing and reading logs using a
//! a ring buffer. The ring buffer stores data using no allocations.
//!
//! # Usage
//! In order the use the log buffer crate, put the following line in your cargo config file:
//!
//! ```ignore
//! log_buffer = "1.0.0"
//! ```
//!
//! # Dependencies
//! This crate has no external dependencies.
//!
//! # Examples
//! A log buffer can use any kind of underlying storage that implements
//! the `AsMut<[u8]> and `AsRef<[u8]>` traits.
//! For example, using a mutable array:
//! ```
//! use log_buffer::LogBuffer;
//!
//!
//! // Using an array.
//! let storage = [0xFF; 32];
//! let log_buffer = LogBuffer::new(storage);
//! ```
//!
//! For another example, using a mutable vector:
//! ```
//! use log_buffer::LogBuffer;
//!
//!
//! // Using a vector.
//! let mut storage = vec![0xFF; 32];
//! let mut log_buffer = LogBuffer::new(storage);
//! ```
//!
//! A final example, using a mutable slice:
//! ```
//! // Using a slice.
//! use log_buffer::LogBuffer;
//!
//!
//! let mut storage = vec![0xFF; 32];
//! let mut log_buffer = LogBuffer::new(&mut storage);
//! ```
//!
//! See the `examples` directory in the source tree for more examples.
//!
mod log_buffer;

pub use self::log_buffer::*;
