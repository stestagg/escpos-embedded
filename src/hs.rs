//! Experimental configuration commands for HS printer modules.
//!
//! These were recovered from the vendor's `PrinterSetting` tool (v3.1.3) rather
//! than any published manual, so treat them as experimental. Settings are
//! stored by the printer and persist across power cycles.

use crate::{Printer, Write};

/// Serial baud rate. The discriminant is the value sent on the wire.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum BaudRate {
    Baud4800 = 0,
    Baud9600 = 1,
    Baud19200 = 2,
    Baud38400 = 3,
    Baud57600 = 4,
    Baud115200 = 5,
    Baud230400 = 6,
    Baud256000 = 7,
    Baud500000 = 8,
    Baud750000 = 9,
    Baud1125000 = 10,
    Baud1500000 = 11,
    Baud460800 = 12,
    Baud921600 = 13,
    Baud1200 = 14,
    Baud2400 = 15,
}

impl BaudRate {
    /// The baud rate in bits per second.
    pub const fn bps(self) -> u32 {
        match self {
            BaudRate::Baud4800 => 4800,
            BaudRate::Baud9600 => 9600,
            BaudRate::Baud19200 => 19200,
            BaudRate::Baud38400 => 38400,
            BaudRate::Baud57600 => 57600,
            BaudRate::Baud115200 => 115200,
            BaudRate::Baud230400 => 230400,
            BaudRate::Baud256000 => 256000,
            BaudRate::Baud500000 => 500000,
            BaudRate::Baud750000 => 750000,
            BaudRate::Baud1125000 => 1125000,
            BaudRate::Baud1500000 => 1500000,
            BaudRate::Baud460800 => 460800,
            BaudRate::Baud921600 => 921600,
            BaudRate::Baud1200 => 1200,
            BaudRate::Baud2400 => 2400,
        }
    }
}

/// Serial parity.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Parity {
    None = 0,
    Odd = 1,
    Even = 2,
}

/// Default font.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum FontType {
    /// 12x24
    A = 0,
    /// 9x24
    B = 1,
    /// 9x17
    C = 2,
    /// 8x16
    D = 3,
}

/// Print density.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Density {
    Light = 0,
    Normal = 1,
    Dark = 2,
}

/// Characters per line for the default font.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum CharsPerLine {
    Chars48 = 0,
    Chars42 = 1,
}

/// Line terminator the printer treats as a line feed.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum LineFeed {
    /// LF (0x0A)
    Lf = 0,
    /// CR (0x0D)
    Cr = 1,
}

/// Default character code page ("Language" in the vendor tool).
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum CodePage {
    Cp437 = 0,
    Katakana = 1,
    Cp850 = 2,
    Cp860 = 3,
    Cp863 = 4,
    Cp865 = 5,
    Wcp1251 = 6,
    Cp866 = 7,
    Mik = 8,
    Cp755 = 9,
    Iran = 10,
    Cp862 = 15,
    Wcp1252 = 16,
    Wcp1253 = 17,
    Cp852 = 18,
    Cp858 = 19,
    IranII = 20,
    Latvian = 21,
    Cp864 = 22,
    Iso8859_1 = 23,
    Cp737 = 24,
    Wcp1257 = 25,
    Thai = 26,
    Cp720 = 27,
    Cp855 = 28,
    Cp857 = 29,
    Wcp1250 = 30,
    Cp775 = 31,
    Wcp1254 = 32,
    Wcp1255 = 33,
    Wcp1256 = 34,
    Wcp1258 = 35,
    Iso8859_2 = 36,
    Iso8859_3 = 37,
    Iso8859_4 = 38,
    Iso8859_5 = 39,
    Iso8859_6 = 40,
    Iso8859_7 = 41,
    Iso8859_8 = 42,
    Iso8859_9 = 43,
    Iso8859_15 = 44,
    Thai2 = 45,
    Cp856 = 46,
    Cp874 = 47,
    Vietnamese = 48,
    EucKr = 251,
    ShiftJis = 252,
    Utf8 = 253,
    TraditionalChinese = 254,
    SimplifiedChinese = 255,
}

/// The vendor tool's "Basic Parameters" page. All fields are sent together.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct BasicSettings {
    pub code_page: CodePage,
    pub baud_rate: BaudRate,
    pub parity: Parity,
    pub font: FontType,
    pub density: Density,
    pub chars_per_line: CharsPerLine,
    pub buzzer: bool,
    pub cash_drawer: bool,
    pub cutter: bool,
    pub line_feed: LineFeed,
    /// Seconds idle before sleeping; 0 disables.
    pub idle_time_s: u16,
    /// Seconds before powering off; 0 disables.
    pub power_off_time_s: u16,
    pub max_feed_length_mm: u16,
    pub page_length_mm: u16,
}

impl Default for BasicSettings {
    fn default() -> Self {
        Self {
            code_page: CodePage::Cp437,
            baud_rate: BaudRate::Baud9600,
            parity: Parity::None,
            font: FontType::A,
            density: Density::Normal,
            chars_per_line: CharsPerLine::Chars48,
            buzzer: false,
            cash_drawer: false,
            cutter: true,
            line_feed: LineFeed::Lf,
            idle_time_s: 0,
            power_off_time_s: 0,
            max_feed_length_mm: 200,
            page_length_mm: 200,
        }
    }
}

impl BasicSettings {
    /// Encode as the `US ( 0x0F` command, the first of the two packets the
    /// vendor tool sends. It omits the line feed and timing fields.
    pub fn encode_command(&self) -> [u8; 17] {
        let payload = [
            0x1F,
            0x73,
            self.baud_rate as u8,
            !self.cutter as u8,
            !self.buzzer as u8,
            !self.cash_drawer as u8,
            self.chars_per_line as u8,
            self.density as u8,
            self.code_page as u8,
            self.parity as u8,
            self.font as u8,
        ];
        let mut packet = [0u8; 17];
        // Length covers the payload plus the trailing checksum.
        packet[..5].copy_from_slice(&[0x1F, 0x28, 0x0F, payload.len() as u8 + 1, 0x00]);
        packet[5..16].copy_from_slice(&payload);
        packet[16] = xor(&payload);
        packet
    }

    /// Encode as the framed protocol packet (`0x03`, command `0xFF`, type
    /// `0x60`), the second of the two packets the vendor tool sends. It omits
    /// the parity, chars per line, buzzer, drawer and cutter fields.
    pub fn encode_packet(&self) -> [u8; 28] {
        let mut data = [0u8; 16];
        data[..4].copy_from_slice(&self.baud_rate.bps().to_be_bytes());
        data[4] = self.code_page as u8;
        data[5] = self.density as u8;
        data[6] = self.font as u8;
        data[7] = self.line_feed as u8;
        data[8..10].copy_from_slice(&self.idle_time_s.to_le_bytes());
        data[10..12].copy_from_slice(&self.power_off_time_s.to_le_bytes());
        data[12..14].copy_from_slice(&self.max_feed_length_mm.to_le_bytes());
        data[14..16].copy_from_slice(&self.page_length_mm.to_le_bytes());

        let mut packet = [0u8; 28];
        // Start, command, type (LE16), parameter (LE32), data length (LE16).
        packet[..10].copy_from_slice(&[0x03, 0xFF, 0x60, 0x00, 0, 0, 0, 0, data.len() as u8, 0]);
        packet[10] = xor(&packet[..10]);
        packet[11] = xor(&data);
        packet[12..].copy_from_slice(&data);
        packet
    }
}

fn xor(data: &[u8]) -> u8 {
    data.iter().fold(0, |acc, b| acc ^ b)
}

impl<T: Write> Printer<T> {
    /// Write the basic settings to the printer's persistent configuration.
    ///
    /// A baud rate change takes effect on the printer side, so the host UART
    /// must be switched to match before talking to it again.
    pub fn hs_set_basic_settings(
        &mut self,
        settings: &BasicSettings,
    ) -> Result<(), <T as Write>::Error> {
        // The vendor tool always sends both; printers act on the one their
        // firmware understands.
        self.transport.write(&settings.encode_command())?;
        self.transport.write(&settings.encode_packet())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_basic_settings() {
        let settings = BasicSettings {
            baud_rate: BaudRate::Baud115200,
            ..BasicSettings::default()
        };
        let expected = [
            0x1F, 0x28, 0x0F, 0x0C, 0x00, // header, length 12
            0x1F, 0x73, 0x05, 0x00, 0x01, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, // payload
            0x68, // xor checksum
        ];
        assert_eq!(settings.encode_command(), expected);

        let expected = [
            0x03, 0xFF, 0x60, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x00, // header
            0x8C, 0xC2, // header and data xor
            0x00, 0x01, 0xC2, 0x00, // 115200
            0x00, 0x01, 0x00, 0x00, // code page, density, font, line feed
            0x00, 0x00, 0x00, 0x00, 0xC8, 0x00, 0xC8,
            0x00, // idle, power off, max feed, page length
        ];
        assert_eq!(settings.encode_packet(), expected);
    }
}
