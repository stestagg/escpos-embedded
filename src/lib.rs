#![no_std]

/// Trait for writing bytes to an underlying transport.
pub trait Write {
    /// Error type produced when writing fails.
    type Error;

    /// Write raw bytes to the transport.
    fn write(&mut self, data: &[u8]) -> Result<(), Self::Error>;
}

/// A simple ESC/POS printer driver.
pub struct Printer<T: Write> {
    transport: T,
}

impl<T: Write> Printer<T> {
    /// Create a new printer from the given transport.
    pub fn new(transport: T) -> Self {
        Self { transport }
    }

    /// Write a line of text followed by a newline character.
    pub fn write_line(&mut self, text: &str) -> Result<(), T::Error> {
        self.transport.write(text.as_bytes())?;
        self.transport.write(b"\n")?;
        Ok(())
    }
}

#[cfg(test)]
extern crate std;

#[cfg(test)]
mod tests {
    use super::*;
    use std::vec::Vec;

    struct MockTransport {
        buffer: Vec<u8>,
    }

    impl MockTransport {
        fn new() -> Self {
            Self { buffer: Vec::new() }
        }
    }

    impl Write for MockTransport {
        type Error = ();

        fn write(&mut self, data: &[u8]) -> Result<(), Self::Error> {
            self.buffer.extend_from_slice(data);
            Ok(())
        }
    }

    #[test]
    fn test_write_line() {
        let mut printer = Printer::new(MockTransport::new());
        printer.write_line("Hello").unwrap();

        assert_eq!(printer.transport.buffer, b"Hello\n".to_vec());
    }
}
