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

#[derive(Debug, PartialEq, Copy, Clone, EnumMeta, ConvertRepr)]
#[repr(u8)]
#[meta_attrs(notation, abbreviation)]
pub enum C1 {
    /// Proposed as a "padding" or "high byte" for single-byte characters to make them two bytes long for easier interoperability with [multiple byte characters](https://www.wikiwand.com/en/articles/Variable-width_encoding).
    /// [Extended Unix Code](https://www.wikiwand.com/en/articles/Extended_Unix_Code) (EUC) occasionally uses this.
    #[meta(notation = "@", abbreviation = "PAD")]
    PaddingCharacter = 0x80,
    /// Proposed to set the high byte of a sequence of [multiple byte characters](https://www.wikiwand.com/en/articles/Variable-width_encoding) so they only need one byte each, as a simple form of data compression.
    #[meta(notation = "A", abbreviation = "HOP")]
    HighOctetPreset = 0x81,
    /// Follows a graphic character where a line break is permitted.
    /// Roughly equivalent to a [soft hyphen](https://www.wikiwand.com/en/articles/Soft_hyphen) or [zero-width](https://www.wikiwand.com/en/articles/Zero-width_space) space except it does not define what is printed at the line break.
    #[meta(notation = "B", abbreviation = "BHP")]
    BreakPermittedHere = 0x82,
    /// Follows the graphic character that is not to be broken. See also [word joiner](https://www.wikiwand.com/en/articles/Word_joiner).
    #[meta(notation = "C", abbreviation = "NBH")]
    NoBreakHere = 0x83,
    /// Move down one line without moving horizontally, to eliminate ambiguity about the meaning of [`LF`](C0::LineFeed).
    #[meta(notation = "D", abbreviation = "IND")]
    Index = 0x84,
    /// Equivalent to [`CR`](C0::CarriageReturn)+[`LF`](C0::LineFeed), to match the [EBCDIC](https://www.wikiwand.com/en/articles/EBCDIC) control character.
    #[meta(notation = "E", abbreviation = "NEL")]
    NextLine = 0x85,
    /// Used by [block-oriented terminals](https://www.wikiwand.com/en/articles/Block-oriented_terminal).
    /// In [xterm](https://www.wikiwand.com/en/articles/Xterm) [`ESC`](C0::Escape)+[`SSA`](C1::StartOfSelectedArea) moves to the lower-left corner of the screen, since certain software assumes this behaviour.
    #[meta(notation = "F", abbreviation = "SSA")]
    StartOfSelectedArea = 0x86,
    /// Used by [block-oriented terminals](https://www.wikiwand.com/en/articles/Block-oriented_terminal).
    #[meta(notation = "G", abbreviation = "ESA")]
    EndOfSelectedArea = 0x87,
    /// Set a tab stop at the current position.
    #[meta(notation = "H", abbreviation = "HTS")]
    CharacterTabulationSet = 0x88,
    /// Right-justify the text since the last tab against the next tab stop.
    #[meta(notation = "I", abbreviation = "HTJ")]
    CharacterTabulationWithJustification = 0x89,
    /// Set a vertical tab stop.
    #[meta(notation = "J", abbreviation = "VTS")]
    LineTabulationSet = 0x8A,
    /// To produce subscripts and superscripts in [ISO/IEC 6429](https://www.wikiwand.com/en/articles/ISO/IEC_6429).
    /// Subscripts use [`PLD`](C1::PartialLineForward)+`text`+[`PLU`](C1::PartialLineBackward) while superscripts use [`PLU`](C1::PartialLineBackward) text [`PLD`](C1::PartialLineForward).
    #[meta(notation = "K", abbreviation = "PLD")]
    PartialLineForward = 0x8B,
    /// To produce subscripts and superscripts in [ISO/IEC 6429](https://www.wikiwand.com/en/articles/ISO/IEC_6429).
    /// Subscripts use [`PLD`](C1::PartialLineForward)+`text`+[`PLU`](C1::PartialLineBackward) while superscripts use [`PLU`](C1::PartialLineBackward) text [`PLD`](C1::PartialLineForward).
    #[meta(notation = "L", abbreviation = "PLU")]
    PartialLineBackward = 0x8C,
    /// Move up one line.
    #[meta(notation = "M", abbreviation = "RI")]
    ReverseLineFeed = 0x8D,
    /// Next character is from the G2 or G3 sets, respectively.
    #[meta(notation = "N", abbreviation = "SS2")]
    SingleShift2 = 0x8E,
    /// Next character is from the G2 or G3 sets, respectively.
    #[meta(notation = "O", abbreviation = "SS3")]
    SingleShift3 = 0x8F,
    /// Followed by a string of printable characters (`0x20` through `0x7E`) and format effectors (`0x08` through `0x0D`), terminated by [`ST`](C1::StringTerminator) (`0x9C`).
    /// [Xterm](https://www.wikiwand.com/en/articles/Xterm) defined a number of these.
    #[meta(notation = "P", abbreviation = "DCS")]
    DeviceControlString = 0x90,
    /// Reserved for private function agreed on between the sender and the recipient of the data.
    #[meta(notation = "Q", abbreviation = "PU1")]
    PrivateUser1 = 0x91,
    /// Reserved for private function agreed on between the sender and the recipient of the data.
    #[meta(notation = "R", abbreviation = "PU2")]
    PrivateUser2 = 0x92,
    ///
    #[meta(notation = "S", abbreviation = "STS")]
    SetTransmitState = 0x93,
    /// Destructive backspace, to eliminate ambiguity about meaning of [`BS`](C0::Backspace).
    #[meta(notation = "T", abbreviation = "CCH")]
    CancelCharacter = 0x94,
    ///
    #[meta(notation = "U", abbreviation = "MW")]
    MessageWaiting = 0x95,
    /// Used by [block-oriented terminals](https://www.wikiwand.com/en/articles/Block-oriented_terminal).
    #[meta(notation = "V", abbreviation = "SPA")]
    StartOfProtectedArea = 0x96,
    /// Used by [block-oriented terminals](https://www.wikiwand.com/en/articles/Block-oriented_terminal).
    #[meta(notation = "W", abbreviation = "EPA")]
    EndOfProtectedArea = 0x97,
    /// Followed by a control string terminated by [`ST`](C1::StringTerminator) (0x9C) which (unlike [`DCS`](C1::DeviceControlString), [`OSC`](C1::OperatingSystemCommand), [`PM`](C1::PrivacyMessage) or [`APC`](C1::ApplicationProgramCommand)) may contain any character except [`SOS`](C1::StartOfString) or [`ST`](C1::StringTerminator).
    #[meta(notation = "X", abbreviation = "SOS")]
    StartOfString = 0x98,
    /// Intended to allow an arbitrary [Unicode](https://www.wikiwand.com/en/articles/Unicode) character to be printed; it would be followed by that character, most likely encoded in [UTF-1](https://www.wikiwand.com/en/articles/UTF-1).
    #[meta(notation = "Y", abbreviation = "SGC")]
    SingleGraphicCharacterIntroducer = 0x99,
    /// To be followed by a single printable character (`0x20` through `0x7E`) or format effector (`0x08` through `0x0D`), and to print it as ASCII no matter what graphic or control sets were in use.
    #[meta(notation = "Z", abbreviation = "SCI")]
    SingleCharacterIntroducer = 0x9A,
    /// Used to introduce control sequences that take parameters. Used for [ANSI escape sequences](https://www.wikiwand.com/en/articles/ANSI_escape_sequences).
    #[meta(notation = "[", abbreviation = "CSI")]
    ControlSequenceIntroducer = 0x9B,
    /// Terminates a string started by [`DCS`](C1::DeviceControlString), [`SOS`](C1::StartOfString), [`OSC`](C1::OperatingSystemCommand), [`PM`](C1::PrivacyMessage) or [`APC`](C1::ApplicationProgramCommand).
    #[meta(notation = "\\", abbreviation = "ST")]
    StringTerminator = 0x9C,
    /// Followed by a string of printable characters (`0x20` through `0x7E`) and format effectors (`0x08` through `0x0D`), terminated by [`ST`](C1::StringTerminator) (0x9C), intended for use to allow in-band signaling of protocol information, but rarely used for that purpose.
    /// Some terminal emulators, including xterm, use [`OSC`](C1::OperatingSystemCommand) sequences for setting the window title and changing the colour palette. They may also support terminating an [`OSC`](C1::OperatingSystemCommand) sequence with [`BEL`](C0::Alert) instead of [`ST`](C1::StringTerminator). Kermit used [`APC`](C1::ApplicationProgramCommand) to transmit commands.
    #[meta(notation = "]", abbreviation = "OSC")]
    OperatingSystemCommand = 0x9D,
    /// Followed by a string of printable characters (`0x20` through `0x7E`) and format effectors (`0x08` through `0x0D`), terminated by [`ST`](C1::StringTerminator) (0x9C), intended for use to allow in-band signaling of protocol information, but rarely used for that purpose.
    /// Some terminal emulators, including xterm, use [`OSC`](C1::OperatingSystemCommand) sequences for setting the window title and changing the colour palette. They may also support terminating an [`OSC`](C1::OperatingSystemCommand) sequence with [`BEL`](C0::Alert) instead of [`ST`](C1::StringTerminator). Kermit used [`APC`](C1::ApplicationProgramCommand) to transmit commands.
    #[meta(notation = "^", abbreviation = "PM")]
    PrivacyMessage = 0x9E,
    /// Followed by a string of printable characters (`0x20` through `0x7E`) and format effectors (`0x08` through `0x0D`), terminated by [`ST`](C1::StringTerminator) (0x9C), intended for use to allow in-band signaling of protocol information, but rarely used for that purpose.
    /// Some terminal emulators, including xterm, use [`OSC`](C1::OperatingSystemCommand) sequences for setting the window title and changing the colour palette. They may also support terminating an [`OSC`](C1::OperatingSystemCommand) sequence with [`BEL`](C0::Alert) instead of [`ST`](C1::StringTerminator). Kermit used [`APC`](C1::ApplicationProgramCommand) to transmit commands.
    #[meta(notation = "_", abbreviation = "APC")]
    ApplicationProgramCommand = 0x9F,
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
