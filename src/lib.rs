#![no_std]

/// Trait for writing bytes to an underlying transport.
pub trait Write {
    /// Error type produced when writing fails.
    type Error;

    /// Write raw bytes to the transport.
    fn write(&mut self, data: &[u8]) -> Result<(), Self::Error>;
}

impl<T: Write + ?Sized> Write for &mut T {
    type Error = T::Error;

    fn write(&mut self, data: &[u8]) -> Result<(), Self::Error> {
        (**self).write(data)
    }
}

/// Trait for reading bytes from an underlying transport.
pub trait Read {
    /// Error type produced when reading fails.
    type Error;

    /// Read bytes into the provided buffer, returning the number of bytes read.
    fn read(&mut self, data: &mut [u8]) -> Result<usize, Self::Error>;
}

impl<T: Read + ?Sized> Read for &mut T {
    type Error = T::Error;

    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        (**self).read(buf)
    }
}

/// A simple ESC/POS printer driver.
pub struct Printer<T: Write> {
    transport: T,
}

#[cfg(feature = "image")]
/// A simple representation of a black & white image.
///
/// The image can either borrow or own the underlying pixel data depending on the
/// type of `D`. Any container that can be referenced as a byte slice (e.g.
/// `&[u8]`, `Vec<u8>`, `[u8; N]`) can be used.
pub struct Image<D>
where
    D: AsRef<[u8]>,
{
    /// Image width in pixels.
    pub width: u16,
    /// Image height in pixels.
    pub height: u16,
    /// Packed bitmap data (row-major, 1 bit per pixel).
    pub data: D,
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

    /// Adapter from an `embedded_io` transport to the crate's own traits.
    pub struct FromEmbeddedIo<T>(pub T);

    impl<T> FromEmbeddedIo<T> {
        pub fn into_inner(self) -> T {
            self.0
        }
    }

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

    impl<T> Write for FromEmbeddedIo<T>
    where
        T: IoWrite,
    {
        type Error = T::Error;

        fn write(&mut self, data: &[u8]) -> Result<(), Self::Error> {
            IoWrite::write_all(&mut self.0, data)
        }
    }

    impl<T> Read for FromEmbeddedIo<T>
    where
        T: IoRead,
    {
        type Error = T::Error;

        fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
            IoRead::read(&mut self.0, buf)
        }
    }
}

#[cfg(feature = "embedded_io")]
pub use embedded_io::FromEmbeddedIo;

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

    /// Set the serial baud rate used by the printer.
    ///
    /// The baud rate value is encoded little-endian in the command sequence.
    pub fn set_baud_rate(&mut self, baud: u32) -> Result<(), <T as Write>::Error> {
        let b = baud.to_le_bytes();
        self.raw(&[
            0x1B, 0x23, 0x23, b'S', b'B', b'D', b'R', b[0], b[1], b[2], b[3],
        ])
    }

    /// Configure the maximum print speed of the printer.
    pub fn set_max_speed(&mut self, speed: u8) -> Result<(), <T as Write>::Error> {
        self.raw(&[0x1B, 0x23, 0x23, b'S', b'T', b'S', b'P', speed])
    }

    /// Enable or disable black mark detection.
    pub fn set_black_mark(&mut self, on: bool) -> Result<(), <T as Write>::Error> {
        let flag = if on { 0x44 } else { 0x66 };
        self.raw(&[0x1F, 0x1B, 0x1F, 0x80, 0x04, 0x05, 0x06, flag])
    }

    #[cfg(feature = "image")]
    /// Print a black & white image using ESC/POS raster format.
    pub fn print_image<D>(&mut self, image: &Image<D>) -> Result<(), <T as Write>::Error>
    where
        D: AsRef<[u8]>,
    {
        let width_bytes = ((image.width + 7) / 8) as u16;
        let x_l = (width_bytes & 0xFF) as u8;
        let x_h = (width_bytes >> 8) as u8;
        let y_l = (image.height & 0xFF) as u8;
        let y_h = (image.height >> 8) as u8;
        // GS v 0 - raster bit image, mode 0
        self.raw(&[0x1D, 0x76, 0x30, 0x00, x_l, x_h, y_l, y_h])?;
        self.transport.write(image.data.as_ref())
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

    #[cfg(feature = "embedded_io")]
    #[test]
    fn test_from_embedded_io() {
        use crate::embedded_io::{Compat, FromEmbeddedIo};
        let mut transport = FromEmbeddedIo(Compat::new(MockTransport::new()));
        Write::write(&mut transport, b"Ok").unwrap();
        let mut buf = [0u8; 2];
        Read::read(&mut transport, &mut buf).unwrap();
        assert_eq!(&buf, b"Ok");
    }

    #[test]
    fn test_write_line() {
        let mut printer = Printer::new(MockTransport::new());
        printer.write_line("Hello").unwrap();

        assert_eq!(printer.transport.buffer, b"Hello\n".to_vec());
    }

    #[cfg(feature = "image")]
    #[test]
    fn test_print_image() {
        let mut printer = Printer::new(MockTransport::new());
        let image = Image {
            width: 8,
            height: 1,
            data: &[0xAA],
        };
        printer.print_image(&image).unwrap();
        let expected = [0x1D, 0x76, 0x30, 0x00, 0x01, 0x00, 0x01, 0x00, 0xAA].to_vec();
        assert_eq!(printer.transport.buffer, expected);
    }

    #[test]
    fn test_set_baud_rate() {
        let mut printer = Printer::new(MockTransport::new());
        printer.set_baud_rate(9600).unwrap();
        let expected = [
            0x1B, 0x23, 0x23, b'S', b'B', b'D', b'R', 0x80, 0x25, 0x00, 0x00,
        ]
        .to_vec();
        assert_eq!(printer.transport.buffer, expected);
    }

    #[test]
    fn test_set_max_speed() {
        let mut printer = Printer::new(MockTransport::new());
        printer.set_max_speed(30).unwrap();
        let expected = [0x1B, 0x23, 0x23, b'S', b'T', b'S', b'P', 0x1E].to_vec();
        assert_eq!(printer.transport.buffer, expected);
    }

    #[test]
    fn test_set_black_mark() {
        let mut printer = Printer::new(MockTransport::new());
        printer.set_black_mark(true).unwrap();
        let expected = [0x1F, 0x1B, 0x1F, 0x80, 0x04, 0x05, 0x06, 0x44].to_vec();
        assert_eq!(printer.transport.buffer, expected);
    }
}
