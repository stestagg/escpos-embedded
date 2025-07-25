#![no_std]

/// Trait for writing bytes to an underlying transport.
pub trait Write {
    /// Error type produced when writing fails.
    type Error;

    /// Write raw bytes to the transport.
    fn write(&mut self, data: &[u8]) -> Result<(), Self::Error>;
}

#[cfg(feature = "embedded_io")]
pub use embedded_io::{Compat, FromEmbeddedIo};

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

/// Paper cutting modes.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CutMode {
    /// Full paper cut.
    Full,
    /// Partial paper cut.
    Partial,
}

impl CutMode {
    fn as_byte(self) -> u8 {
        match self {
            CutMode::Full => 0x00,
            CutMode::Partial => 0x01,
        }
    }
}

/// Underline styles.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UnderlineMode {
    /// No underline.
    None,
    /// Single underline.
    Single,
    /// Double underline.
    Double,
}

impl UnderlineMode {
    fn as_byte(self) -> u8 {
        match self {
            UnderlineMode::None => 0x00,
            UnderlineMode::Single => 0x01,
            UnderlineMode::Double => 0x02,
        }
    }
}

/// Horizontal alignment modes.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Align {
    Left,
    Center,
    Right,
}

impl Align {
    fn as_byte(self) -> u8 {
        match self {
            Align::Left => 0x00,
            Align::Center => 0x01,
            Align::Right => 0x02,
        }
    }
}

/// Font type selection.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Font {
    FontA,
    FontB,
}

impl Font {
    fn as_byte(self) -> u8 {
        match self {
            Font::FontA => 0x00,
            Font::FontB => 0x01,
        }
    }
}

/// Text justification.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Justification {
    Left,
    Center,
    Right,
}

impl Justification {
    fn as_byte(self) -> u8 {
        match self {
            Justification::Left => 0x00,
            Justification::Center => 0x01,
            Justification::Right => 0x02,
        }
    }
}

/// Print density levels.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Density {
    Level0,
    Level1,
    Level2,
    Level3,
    Level4,
    Level5,
    Level6,
    Level7,
    Level8,
}

impl Density {
    fn as_byte(self) -> u8 {
        match self {
            Density::Level0 => 0x00,
            Density::Level1 => 0x01,
            Density::Level2 => 0x02,
            Density::Level3 => 0x03,
            Density::Level4 => 0x04,
            Density::Level5 => 0x05,
            Density::Level6 => 0x06,
            Density::Level7 => 0x07,
            Density::Level8 => 0x08,
        }
    }
}

/// Printer speed options.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PrintSpeed {
    Speed1,
    Speed2,
    Speed3,
    Speed4,
}

impl PrintSpeed {
    fn as_byte(self) -> u8 {
        match self {
            PrintSpeed::Speed1 => 0x00,
            PrintSpeed::Speed2 => 0x01,
            PrintSpeed::Speed3 => 0x02,
            PrintSpeed::Speed4 => 0x03,
        }
    }
}

#[cfg(feature = "embedded_io")]
mod embedded_io {
    use super::{Read, Write};
    use embedded_io::{ErrorType as IoErrorType, Read as IoRead, Write as IoWrite};

    /// Wrapper type that provides `embedded_io` compatibility for a transport.
    pub struct Compat<T>(pub T);

    /// Adapter that converts an `embedded_io` transport into the traits used by this crate.
    pub struct FromEmbeddedIo<T>(pub T);

    impl<T> FromEmbeddedIo<T> {
        pub fn new(inner: T) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> T {
            self.0
        }
    }

    impl<T> Compat<T> {
        pub fn new(inner: T) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> T {
            self.0
        }
    }

    impl<T> IoErrorType for Compat<T>
    where
        T: Write,
        <T as Write>::Error: embedded_io::Error,
    {
        type Error = <T as Write>::Error;
    }

    impl<T> IoRead for Compat<T>
    where
        T: Read<Error = <T as Write>::Error> + Write,
        <T as Write>::Error: embedded_io::Error,
    {
        fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
            self.0.read(buf)
        }
    }

    impl<T> IoWrite for Compat<T>
    where
        T: Write,
        <T as Write>::Error: embedded_io::Error,
    {
        fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
            self.0.write(buf)?;
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    impl<T> Read for FromEmbeddedIo<T>
    where
        T: IoRead,
    {
        type Error = <T as IoErrorType>::Error;

        fn read(&mut self, data: &mut [u8]) -> Result<usize, Self::Error> {
            IoRead::read(&mut self.0, data)
        }
    }

    impl<T> Write for FromEmbeddedIo<T>
    where
        T: IoWrite,
    {
        type Error = <T as IoErrorType>::Error;

        fn write(&mut self, data: &[u8]) -> Result<(), Self::Error> {
            IoWrite::write_all(&mut self.0, data)
        }
    }
}

impl<T: Write> Printer<T> {
    /// Create a new printer from the given transport.
    pub fn new(transport: T) -> Self {
        Self { transport }
    }
}

impl<T> Printer<T>
where
    T: Write + Read<Error = <T as Write>::Error>,
{
    /// Write raw text to the printer.
    pub fn write(&mut self, text: &str) -> Result<(), <T as Write>::Error> {
        self.transport.write(text.as_bytes())
    }

    /// Write text followed by a newline.
    pub fn write_line(&mut self, text: &str) -> Result<(), <T as Write>::Error> {
        self.write(text)?;
        self.transport.write(b"\n")
    }

    /// Feed the specified number of lines.
    pub fn feed(&mut self, lines: u8) -> Result<(), <T as Write>::Error> {
        self.raw(&[0x1B, 0x64, lines])
    }

    /// Cut the paper using the given mode.
    pub fn cut(&mut self, mode: CutMode) -> Result<(), <T as Write>::Error> {
        self.raw(&[0x1D, 0x56, mode.as_byte()])
    }

    /// Enable or disable bold mode.
    pub fn set_bold(&mut self, on: bool) -> Result<(), <T as Write>::Error> {
        let flag = if on { 0x01 } else { 0x00 };
        self.raw(&[0x1B, 0x45, flag])
    }

    /// Set underline mode.
    pub fn set_underline(&mut self, mode: UnderlineMode) -> Result<(), <T as Write>::Error> {
        self.raw(&[0x1B, 0x2D, mode.as_byte()])
    }

    /// Set text alignment.
    pub fn set_align(&mut self, align: Align) -> Result<(), <T as Write>::Error> {
        self.raw(&[0x1B, 0x61, align.as_byte()])
    }

    /// Select printer font.
    pub fn set_font(&mut self, font: Font) -> Result<(), <T as Write>::Error> {
        self.raw(&[0x1B, 0x4D, font.as_byte()])
    }

    /// Set character size using width and height multipliers.
    pub fn set_size(&mut self, width: u8, height: u8) -> Result<(), <T as Write>::Error> {
        let width = core::cmp::min(width, 7);
        let height = core::cmp::min(height, 7);
        let param = (width << 4) | height;
        self.raw(&[0x1D, 0x21, param])
    }

    /// Enable or disable inverted printing.
    pub fn set_invert(&mut self, on: bool) -> Result<(), <T as Write>::Error> {
        let flag = if on { 0x01 } else { 0x00 };
        self.raw(&[0x1D, 0x42, flag])
    }

    /// Set text justification.
    pub fn set_justification(&mut self, mode: Justification) -> Result<(), <T as Write>::Error> {
        self.raw(&[0x1B, 0x61, mode.as_byte()])
    }

    /// Set print density level.
    pub fn set_density(&mut self, level: Density) -> Result<(), <T as Write>::Error> {
        self.raw(&[0x1D, 0x7C, level.as_byte()])
    }

    /// Set print speed.
    pub fn set_print_speed(&mut self, speed: PrintSpeed) -> Result<(), <T as Write>::Error> {
        self.raw(&[0x1F, 0x50, speed.as_byte()])
    }

    /// Send raw bytes directly to the printer.
    pub fn raw(&mut self, data: &[u8]) -> Result<(), <T as Write>::Error> {
        self.transport.write(data)
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
        type Error = core::convert::Infallible;

        fn write(&mut self, data: &[u8]) -> Result<(), Self::Error> {
            self.buffer.extend_from_slice(data);
            Ok(())
        }
    }

    impl Read for MockTransport {
        type Error = core::convert::Infallible;

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
        ::embedded_io::Write::write_all(&mut transport, b"Hi").unwrap();
        let mut buf = [0u8; 2];
        ::embedded_io::Read::read_exact(&mut transport, &mut buf).unwrap();
        assert_eq!(&buf, b"Hi");
    }

    #[test]
    fn test_write_line() {
        let mut printer = Printer::new(MockTransport::new());
        printer.write_line("Hello").unwrap();

        assert_eq!(printer.transport.buffer, b"Hello\n".to_vec());
    }
}
