use std::fmt;
use std::str;


#[derive(Debug)]
pub struct LogBuffer<Storage> {
    buffer: Storage,
    wrapped: bool,
    start: usize,
    end: usize,
}

impl<Storage> LogBuffer<Storage> where Storage: AsRef<[u8]> + AsMut<[u8]> {
    pub fn new(storage: Storage) -> LogBuffer<Storage> {
        let mut log_buffer = LogBuffer {
            buffer: storage,
            wrapped: false,
            start: 0,
            end: 0,
        };

        log_buffer.clear();
        log_buffer
    }

    pub fn clear(&mut self) {
        self.wrapped = false;
        self.start = 0;
        self.end = 0;
        for byte in self.buffer.as_mut().iter_mut() {
            *byte = 0x00;
        }
    }

    pub fn is_empty(&self) -> bool {
        (self.start == self.end) && !self.wrapped
    }

    pub fn len(&self) -> usize {
        if self.start < self.end {
            (self.start - self.end) + 1
        } else if self.start > self.end {
            (self.end - self.start) + 1
        } else if self.wrapped && self.start == self.end {
            self.buffer.as_ref().len()
        } else {
            0
        }
    }

    pub fn is_full(&self) -> bool {
        self.len() == self.capacity()
    }

    pub fn capacity(&self) -> usize {
        self.buffer.as_ref().len()
    }

    fn rotate(&mut self) {
        if self.wrapped && (self.start == self.end) {
            self.buffer.as_mut().rotate_left(self.end);
            self.wrapped = false;
            self.start = 0;
            self.end = self.buffer.as_ref().len();
        } else if self.start < self.end {
            self.buffer.as_mut().rotate_left(self.start);
            self.wrapped = false;
            self.end -= self.start;
            self.start = 0;
        } else if self.start > self.end {
            self.buffer.as_mut().rotate_left(self.end);
            self.wrapped = false;
            self.start -= self.end;
            self.end = self.buffer.as_ref().len();
            self.buffer.as_mut().rotate_left(self.start);
            self.end -= self.start;
            self.start = 0;
        } else {
            self.buffer.as_mut().rotate_left(self.start);
            self.wrapped = false;
            self.start = 0;
            self.end = 0;
        }
    }

    pub fn extract(&mut self) -> &str {
        fn is_utf8_leader(byte: u8) -> bool {
            byte & 0b10000000 == 0b00000000 || byte & 0b11100000 == 0b11000000 ||
            byte & 0b11110000 == 0b11100000 || byte & 0b11111000 == 0b11110000
        }

        self.rotate();

        let buffer = self.buffer.as_mut();
        let start = self.start;
        let end = self.end;
        for i in start..end {
            if is_utf8_leader(buffer[i]) {
                return str::from_utf8(&buffer[i..end]).unwrap();
            }
        }

        ""
    }
}

impl<Storage> fmt::Write for LogBuffer<Storage> where Storage: AsRef<[u8]> + AsMut<[u8]> {
    fn write_str(&mut self, st: &str) -> fmt::Result {
        for &byte in st.as_bytes() {
            self.buffer.as_mut()[self.end] = byte;
            self.end += 1;
            if self.end >= self.buffer.as_ref().len() {
                self.wrapped = true;
            }
            self.end %= self.buffer.as_mut().len();
        }

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::LogBuffer;
    use std::fmt::Write;


    #[test]
    fn log_buffer_successive_rotate_operations_should_leave_internal_state_unchanged() {
        let mut log_buffer = LogBuffer::new([0xFF; 16]);
        write!(log_buffer, "abcdefgh").unwrap();

        log_buffer.rotate();
        let wrapped = log_buffer.wrapped;
        let start = log_buffer.start;
        let end = log_buffer.end;
        let storage = log_buffer.buffer;
        log_buffer.rotate();

        assert_eq!(log_buffer.wrapped, wrapped);
        assert_eq!(log_buffer.start, start);
        assert_eq!(log_buffer.end, end);
        assert_eq!(log_buffer.buffer, storage);
    }
}
