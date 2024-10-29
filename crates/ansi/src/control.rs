use enum_meta::EnumMeta;

/// C0 control codes following [ISO/IEC 2022](https://en.wikipedia.org/wiki/ISO/IEC_2022)
/// specification.
#[derive(Debug, PartialEq, EnumMeta)]
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
    Delete = 0x7F
}

impl TryFrom<u8> for C0 {
    type Error = u8;

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        use C0::*;

        match byte {
            0x00 => Ok(Null),
            0x01 => Ok(StartOfHeading),
            0x02 => Ok(StartOfText),
            0x03 => Ok(EndOfText),
            0x04 => Ok(EndOfTransmission),
            0x05 => Ok(Enquiry),
            0x06 => Ok(Acknowledge),
            0x07 => Ok(Alert),
            0x08 => Ok(Backspace),
            0x09 => Ok(CharacterTabulation),
            0x0A => Ok(LineFeed),
            0x0B => Ok(LineTabulation),
            0x0C => Ok(FormFeed),
            0x0D => Ok(CarriageReturn),
            0x0E => Ok(ShiftOut),
            0x0F => Ok(ShiftIn),
            0x10 => Ok(DataLinkEscape),
            0x11 => Ok(DeviceControlOne),
            0x12 => Ok(DeviceControlTwo),
            0x13 => Ok(DeviceControlThree),
            0x14 => Ok(DeviceControlFour),
            0x15 => Ok(NegativeAcknowledge),
            0x16 => Ok(SynchronousIdle),
            0x17 => Ok(EndOfTransmissionBlock),
            0x18 => Ok(Cancel),
            0x19 => Ok(EndOfMedium),
            0x1A => Ok(Substitute),
            0x1B => Ok(Escape),
            0x1C => Ok(FileSeparator),
            0x1D => Ok(GroupSeparator),
            0x1E => Ok(RecordSeparator),
            0x1F => Ok(UnitSeparator),
            byte => Err(byte),
        }
    }
}

impl From<C0> for u8 {
    fn from(c0: C0) -> Self {
        use C0::*;

        match c0 {
            Null => 0x00,
            StartOfHeading => 0x01,
            StartOfText => 0x02,
            EndOfText => 0x03,
            EndOfTransmission => 0x04,
            Enquiry => 0x05,
            Acknowledge => 0x06,
            Alert => 0x07,
            Backspace => 0x08,
            CharacterTabulation => 0x09,
            LineFeed => 0x0A,
            LineTabulation => 0x0B,
            FormFeed => 0x0C,
            CarriageReturn => 0x0D,
            ShiftOut => 0x0E,
            ShiftIn => 0x0F,
            DataLinkEscape => 0x10,
            DeviceControlOne => 0x11,
            DeviceControlTwo => 0x12,
            DeviceControlThree => 0x13,
            DeviceControlFour => 0x14,
            NegativeAcknowledge => 0x15,
            SynchronousIdle => 0x16,
            EndOfTransmissionBlock => 0x17,
            Cancel => 0x18,
            EndOfMedium => 0x19,
            Substitute => 0x1A,
            Escape => 0x1B,
            FileSeparator => 0x1C,
            GroupSeparator => 0x1D,
            RecordSeparator => 0x1E,
            UnitSeparator => 0x1F,
        }
    }
}
