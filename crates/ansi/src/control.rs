use std::fmt::{self, Display, LowerHex, UpperHex};

use enum_meta::EnumMeta;
use enum_repr_convert::ConvertRepr;

/// C0 control codes following [ISO/IEC 2022](https://en.wikipedia.org/wiki/ISO/IEC_2022)
/// specification.
#[derive(Debug, PartialEq, Copy, Clone, EnumMeta, ConvertRepr)]
#[repr(u8)]
#[meta_attrs(caret_notation, abbreviation)]
pub enum C0 {
    /// Does nothing. The code of blank paper tape, and also used for padding to slow transmission.
    #[meta(caret_notation = "^@", abbreviation = "NUL")]
    Null = 0x00,
    /// First character of the heading of a message.
    #[meta(caret_notation = "^A", abbreviation = "SOH")]
    StartOfHeading = 0x01,
    /// Terminates the header and starts the message text.
    #[meta(caret_notation = "^B", abbreviation = "STX")]
    StartOfText = 0x02,
    /// Ends the message text, starts a footer (up to the next TC character).
    #[meta(caret_notation = "^C", abbreviation = "ETX")]
    EndOfText = 0x03,
    /// Ends the transmission of one or more messages. May place terminals on standby.
    #[meta(caret_notation = "^D", abbreviation = "EOT")]
    EndOfTransmission = 0x04,
    /// Trigger a response at the receiving end, to see if it is still present.
    #[meta(caret_notation = "^E", abbreviation = "ENQ")]
    Enquiry = 0x05,
    /// Indication of successful receipt of a message.
    #[meta(caret_notation = "^F", abbreviation = "ACK")]
    Acknowledge = 0x06,
    /// Call for attention from an operator.
    #[meta(caret_notation = "^G", abbreviation = "BEL")]
    Alert = 0x07,
    /// Move one position leftwards. Next character may overprint or replace the character that was there.
    #[meta(caret_notation = "^H", abbreviation = "BS")]
    Backspace = 0x08,
    /// Move right to the next tab stop.
    #[meta(caret_notation = "^I", abbreviation = "HT")]
    CharacterTabulation = 0x09,
    /// Move down to the same position on the next line (some devices also moved to the left column).
    #[meta(caret_notation = "^J", abbreviation = "LF")]
    LineFeed = 0x0A,
    /// Move down to the next vertical tab stop.
    #[meta(caret_notation = "^K", abbreviation = "VT")]
    LineTabulation = 0x0B,
    /// Move down to the top of the next page.
    #[meta(caret_notation = "^L", abbreviation = "FF")]
    FormFeed = 0x0C,
    /// Move to column zero while staying on the same line.
    #[meta(caret_notation = "^M", abbreviation = "CR")]
    CarriageReturn = 0x0D,
    /// Switch to an alternative character set.
    #[meta(caret_notation = "^N", abbreviation = "SO")]
    ShiftOut = 0x0E,
    /// Return to regular character set after SO.
    #[meta(caret_notation = "^O", abbreviation = "SI")]
    ShiftIn = 0x0F,
    /// Cause a limited number of contiguously following characters to be interpreted in some different way.
    #[meta(caret_notation = "^P", abbreviation = "DLE")]
    DataLinkEscape = 0x10,
    /// Turn on (DC1 and DC2) or off (DC3 and DC4) devices.
    /// Teletype used these for the paper tape reader and the paper tape punch. The first use became the de facto standard for software flow control.
    #[meta(caret_notation = "^Q", abbreviation = "XON")]
    DeviceControlOne = 0x11,
    /// Turn on (DC1 and DC2) or off (DC3 and DC4) devices.
    /// Teletype used these for the paper tape reader and the paper tape punch. The first use became the de facto standard for software flow control.
    #[meta(caret_notation = "^R", abbreviation = "TAPE")]
    DeviceControlTwo = 0x12,
    /// Turn on (DC1 and DC2) or off (DC3 and DC4) devices.
    /// Teletype used these for the paper tape reader and the paper tape punch. The first use became the de facto standard for software flow control.
    #[meta(caret_notation = "^S", abbreviation = "XOFF")]
    DeviceControlThree = 0x13,
    /// Turn on (DC1 and DC2) or off (DC3 and DC4) devices.
    /// Teletype used these for the paper tape reader and the paper tape punch. The first use became the de facto standard for software flow control.
    #[meta(caret_notation = "^T", abbreviation = "TAPE")]
    DeviceControlFour = 0x14,
    /// Negative response to a sender, such as a detected error.
    #[meta(caret_notation = "^U", abbreviation = "NAK")]
    NegativeAcknowledge = 0x15,
    /// Sent in synchronous transmission systems when no other character is being transmitted.
    #[meta(caret_notation = "^V", abbreviation = "SYN")]
    SynchronousIdle = 0x16,
    /// End of a transmission block of data when data are divided into such blocks for transmission purposes.
    #[meta(caret_notation = "^W", abbreviation = "ETB")]
    EndOfTransmissionBlock = 0x17,
    /// Indicates that the data preceding it are in error or are to be disregarded.
    #[meta(caret_notation = "^X", abbreviation = "CAN")]
    Cancel = 0x18,
    /// Indicates on paper or magnetic tapes that the end of the usable portion of the tape had been reached.
    #[meta(caret_notation = "^Y", abbreviation = "EM")]
    EndOfMedium = 0x19,
    /// Replaces a character that was found to be invalid or in error. Should be ignored.
    #[meta(caret_notation = "^Z", abbreviation = "SUB")]
    Substitute = 0x1A,
    /// Alters the meaning of a limited number of following bytes.
    /// Nowadays this is almost always used to introduce an ANSI escape sequence.
    #[meta(caret_notation = "^[", abbreviation = "ESC")]
    Escape = 0x1B,
    /// Can be used as delimiters to mark fields of data structures.
    /// US is the lowest level, while RS, GS, and FS are of increasing level to divide groups made up of items of the level beneath it.
    /// SP (space) could be considered an even lower level.
    #[meta(caret_notation = "^\\", abbreviation = "FS")]
    FileSeparator = 0x1C,
    /// Can be used as delimiters to mark fields of data structures.
    /// US is the lowest level, while RS, GS, and FS are of increasing level to divide groups made up of items of the level beneath it.
    /// SP (space) could be considered an even lower level.
    #[meta(caret_notation = "^]", abbreviation = "GS")]
    GroupSeparator = 0x1D,
    /// Can be used as delimiters to mark fields of data structures.
    /// US is the lowest level, while RS, GS, and FS are of increasing level to divide groups made up of items of the level beneath it.
    /// SP (space) could be considered an even lower level.
    #[meta(caret_notation = "^^", abbreviation = "RS")]
    RecordSeparator = 0x1E,
    /// Can be used as delimiters to mark fields of data structures.
    /// US is the lowest level, while RS, GS, and FS are of increasing level to divide groups made up of items of the level beneath it.
    /// SP (space) could be considered an even lower level.
    #[meta(caret_notation = "^_", abbreviation = "US")]
    UnitSeparator = 0x1F,
    /// Move right one character position.
    /// Technically not part of C0 range
    #[meta(caret_notation = " ", abbreviation = "SP")]
    Space = 0x20,
    /// Should be ignored. Used to delete characters on punched tape by punching out all the holes.
    /// Technically not part of C0 range
    #[meta(caret_notation = "^?", abbreviation = "DEL")]
    Delete = 0x7F,
}

impl Display for C0 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.caret_notation())
    }
}

impl LowerHex for C0 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value: u8 = (*self).into();
        write!(f, "{:x}", value)
    }
}

impl UpperHex for C0 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value: u8 = (*self).into();
        write!(f, "{:X}", value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn caret_notation() {
        let c0 = C0::Null;
        let caret_notation = c0.caret_notation();

        assert_eq!(caret_notation, "^@");
    }

    #[test]
    fn abbreviation() {
        let c0 = C0::Null;
        let abbreviation = c0.abbreviation();

        assert_eq!(abbreviation, "NUL");
    }

    #[test]
    fn into_u8() {
        let c0 = C0::Null;
        let byte: u8 = c0.into();

        assert_eq!(byte, 0x00);
    }

    #[test]
    fn from_byte_valid() {
        let c0_res: Result<C0, u8> = 0x00.try_into();

        assert_eq!(c0_res, Ok(C0::Null));
    }

    #[test]
    fn from_byte_invalid() {
        let c0_res: Result<C0, u8> = 0xFF.try_into();

        assert_eq!(c0_res, Err(0xFF));
    }
}
