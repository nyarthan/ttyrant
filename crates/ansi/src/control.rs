use enum_meta::EnumMeta;
use enum_repr_convert::ConvertRepr;

/// C0 control codes following [ISO/IEC 2022](https://en.wikipedia.org/wiki/ISO/IEC_2022)
/// specification.
#[derive(Debug, PartialEq, EnumMeta, ConvertRepr)]
#[repr(u8)]
#[meta_attrs(caret_notation, abbreviation)]
pub enum C0 {
    #[meta(caret_notation = "^@", abbreviation = "NUL")]
    Null = 0x00,
    #[meta(caret_notation = "^A", abbreviation = "SOH")]
    StartOfHeading = 0x01,
    #[meta(caret_notation = "^B", abbreviation = "STX")]
    StartOfText = 0x02,
    #[meta(caret_notation = "^C", abbreviation = "ETX")]
    EndOfText = 0x03,
    #[meta(caret_notation = "^D", abbreviation = "EOT")]
    EndOfTransmission = 0x04,
    #[meta(caret_notation = "^E", abbreviation = "ENQ")]
    Enquiry = 0x05,
    #[meta(caret_notation = "^F", abbreviation = "ACK")]
    Acknowledge = 0x06,
    #[meta(caret_notation = "^G", abbreviation = "BEL")]
    Alert = 0x07,
    #[meta(caret_notation = "^H", abbreviation = "BS")]
    Backspace = 0x08,
    #[meta(caret_notation = "^I", abbreviation = "HT")]
    CharacterTabulation = 0x09,
    #[meta(caret_notation = "^J", abbreviation = "LF")]
    LineFeed = 0x0A,
    #[meta(caret_notation = "^K", abbreviation = "VT")]
    LineTabulation = 0x0B,
    #[meta(caret_notation = "^L", abbreviation = "FF")]
    FormFeed = 0x0C,
    #[meta(caret_notation = "^M", abbreviation = "CR")]
    CarriageReturn = 0x0D,
    #[meta(caret_notation = "^N", abbreviation = "SO")]
    ShiftOut = 0x0E,
    #[meta(caret_notation = "^O", abbreviation = "SI")]
    ShiftIn = 0x0F,
    #[meta(caret_notation = "^P", abbreviation = "DLE")]
    DataLinkEscape = 0x10,
    #[meta(caret_notation = "^Q", abbreviation = "XON")]
    DeviceControlOne = 0x11,
    #[meta(caret_notation = "^R", abbreviation = "TAPE")]
    DeviceControlTwo = 0x12,
    #[meta(caret_notation = "^S", abbreviation = "XOFF")]
    DeviceControlThree = 0x13,
    #[meta(caret_notation = "^T", abbreviation = "TAPE")]
    DeviceControlFour = 0x14,
    #[meta(caret_notation = "^U", abbreviation = "NAK")]
    NegativeAcknowledge = 0x15,
    #[meta(caret_notation = "^V", abbreviation = "SYN")]
    SynchronousIdle = 0x16,
    #[meta(caret_notation = "^W", abbreviation = "ETB")]
    EndOfTransmissionBlock = 0x17,
    #[meta(caret_notation = "^X", abbreviation = "CAN")]
    Cancel = 0x18,
    #[meta(caret_notation = "^Y", abbreviation = "EM")]
    EndOfMedium = 0x19,
    #[meta(caret_notation = "^Z", abbreviation = "SUB")]
    Substitute = 0x1A,
    #[meta(caret_notation = "^[", abbreviation = "ESC")]
    Escape = 0x1B,
    #[meta(caret_notation = "^\\", abbreviation = "FS")]
    FileSeparator = 0x1C,
    #[meta(caret_notation = "^]", abbreviation = "GS")]
    GroupSeparator = 0x1D,
    #[meta(caret_notation = "^^", abbreviation = "RS")]
    RecordSeparator = 0x1E,
    #[meta(caret_notation = "^_", abbreviation = "US")]
    UnitSeparator = 0x1F,
    /// Technically not part of C0 range
    #[meta(caret_notation = " ", abbreviation = "SP")]
    Space = 0x20,
    /// Technically not part of C0 range
    #[meta(caret_notation = "^?", abbreviation = "DEL")]
    Delete = 0x7F,
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
