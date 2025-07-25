#![no_std]

/// Trait for writing bytes to an underlying transport.
pub trait Write {
    /// Error type produced when writing fails.
    type Error;

    /// Write raw bytes to the transport.
    fn write(&mut self, data: &[u8]) -> Result<(), Self::Error>;
}

/// Trait for reading bytes from an underlying transport.
pub trait Read {
    /// Error type produced when reading fails.
    type Error;

    /// Read bytes into the provided buffer, returning the number of bytes read.
    fn read(&mut self, data: &mut [u8]) -> Result<usize, Self::Error>;
}

/// A simple ESC/POS printer driver.
pub struct Printer<T: Write> {
    transport: T,
}

#[cfg(feature = "embedded_io")]
mod embedded_io {
    use super::{Read, Write};
    use embedded_io::{Read as IoRead, Write as IoWrite};

    /// Wrapper type that provides `embedded_io` compatibility for a transport.
    pub struct Compat<T>(pub T);

    impl<T> Compat<T> {
        pub fn new(inner: T) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> T {
            self.0
        }
    }

    impl<T: Read> IoRead for Compat<T> {
        type Error = T::Error;

        fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
            self.0.read(buf)
        }
    }

    impl<T: Write> IoWrite for Compat<T> {
        type Error = T::Error;

        fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
            self.0.write(buf)?;
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<(), Self::Error> {
            Ok(())
        }
    }
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

    impl Read for MockTransport {
        type Error = ();

        fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
            let len = core::cmp::min(buf.len(), self.buffer.len());
            buf[..len].copy_from_slice(&self.buffer[..len]);
            self.buffer.drain(..len);
            Ok(len)
        }
    }

    #[cfg(feature = "embedded_io")]
    #[test]
    fn test_embedded_io_compat() {
        use crate::embedded_io::Compat;
        let mut transport = Compat::new(MockTransport::new());
        embedded_io::Write::write_all(&mut transport, b"Hi").unwrap();
        let mut buf = [0u8; 2];
        embedded_io::Read::read_exact(&mut transport, &mut buf).unwrap();
        assert_eq!(&buf, b"Hi");
    }

    #[test]
    fn test_write_line() {
        let mut printer = Printer::new(MockTransport::new());
        printer.write_line("Hello").unwrap();

        assert_eq!(printer.transport.buffer, b"Hello\n".to_vec());
    }
}
