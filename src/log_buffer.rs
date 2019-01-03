use core::fmt;
use core::str;


///
/// A `LogBuffer` is a ring buffer for storing UTF-8 strings for logging.
/// It logs data using zero allocations.
///
#[derive(Debug)]
pub struct LogBuffer<Storage> {
    /// the underlying storage for the ring buffer.
    storage: Storage,
    /// Whether the ring buffer has wrapped around since its last call to rotate.
    wrapped: bool,
    /// The position of the next available byte in the ring buffer.
    end: usize,
}

impl<Storage> LogBuffer<Storage> where Storage: AsRef<[u8]> + AsMut<[u8]> {
    ///
    /// Construct a new log buffer.
    ///
    pub fn new(storage: Storage) -> LogBuffer<Storage> {
        let mut log_buffer = LogBuffer {
            storage: storage,
            wrapped: false,
            end: 0,
        };

        log_buffer.clear();
        log_buffer
    }

    ///
    /// Empty out the log buffer.
    ///
    pub fn clear(&mut self) {
        self.wrapped = false;
        self.end = 0;
        for byte in self.storage.as_mut().iter_mut() {
            *byte = 0x00;
        }
    }

    ///
    /// Determine whether the ring buffer is empty, i.e. it contains no data.
    ///
    pub fn is_empty(&self) -> bool {
        (self.end == 0) && !self.wrapped
    }

    ///
    /// Determine the number of bytes currently stored in the log buffer.
    ///
    pub fn len(&self) -> usize {
        if self.wrapped {
            self.storage.as_ref().len()
        } else {
            self.end
        }
    }

    ///
    /// Calculate the amount of space in bytes remaining in the ring buffer.
    ///
    pub fn space_remaining(&self) -> usize {
        self.capacity() - self.len()
    }

    ///
    /// Determine whether the ring buffer is full.
    ///
    pub fn is_full(&self) -> bool {
        self.space_remaining() == 0
    }

    ///
    /// The maximum number of bytes that a log buffer can store. This is not
    /// the same as the number of UTF-8 characters the buffer can store since
    /// UTF-8 characters can have multiple code points.
    ///
    pub fn capacity(&self) -> usize {
        self.storage.as_ref().len()
    }

    fn rotate(&mut self) {
        if self.wrapped {
            self.storage.as_mut().rotate_left(self.end);
            self.end = self.len();
            self.wrapped = false;
        }
    }

    ///
    /// Extract a string slice from the ring buffer. This is a zero allocation operation.
    ///
    pub fn extract(&mut self) -> &str {
        fn is_utf8_leader(byte: u8) -> bool {
            byte & 0b10000000 == 0b00000000 || byte & 0b11100000 == 0b11000000 ||
            byte & 0b11110000 == 0b11100000 || byte & 0b11111000 == 0b11110000
        }

        self.rotate();

        let buffer = self.storage.as_mut();
        let end = self.end;
        for i in 0..end {
            if is_utf8_leader(buffer[i]) {
                return str::from_utf8(&buffer[i..end]).unwrap();
            }
        }

        ""
    }
}

impl<Storage> fmt::Write for LogBuffer<Storage> where Storage: AsRef<[u8]> + AsMut<[u8]> {
    ///
    /// Write a UTF-8 string into the ring buffer.
    ///
    fn write_str(&mut self, st: &str) -> fmt::Result {
        for &byte in st.as_bytes() {
            self.storage.as_mut()[self.end] = byte;
            self.end += 1;
            if self.end >= self.storage.as_ref().len() {
                self.wrapped = true;
            }
            self.end %= self.storage.as_mut().len();
        }

        Ok(())
    }
}


#[cfg(test)]
mod rotate_tests {
    use super::LogBuffer;
    use core::fmt::Write;


    #[test]
    fn log_buffer_successive_rotate_operations_should_leave_internal_state_unchanged() {
        let mut log_buffer = LogBuffer::new([0xFF; 16]);
        write!(log_buffer, "abcdefgh").unwrap();

        log_buffer.rotate();
        let wrapped = log_buffer.wrapped;
        let end = log_buffer.end;
        let storage = log_buffer.storage;
        log_buffer.rotate();
        log_buffer.rotate();

        assert_eq!(log_buffer.wrapped, wrapped);
        assert_eq!(log_buffer.end, end);
        assert_eq!(log_buffer.storage, storage);
    }

    #[test]
    fn log_buffer_rotate_should_unwrap_buffer() {
        let mut log_buffer = LogBuffer::new([0xFF; 16]);
        write!(log_buffer, "abcdefghijklmnop").unwrap();

        assert_eq!(log_buffer.wrapped, true);
        log_buffer.rotate();
        assert_eq!(log_buffer.wrapped, false);
    }

    #[test]
    fn log_buffer_rotate_should_unwrap_end() {
        let mut log_buffer = LogBuffer::new([0xFF; 16]);
        write!(log_buffer, "abcdefghijklmnop").unwrap();

        let end_before_rotate = log_buffer.end;
        log_buffer.rotate();
        assert!(log_buffer.end > end_before_rotate);
    }
}
