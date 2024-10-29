/// C0 control codes following [ISO/IEC 2022](https://en.wikipedia.org/wiki/ISO/IEC_2022)
/// specification.
#[derive(Debug, PartialEq)]
pub enum C0 {
    Null,
    StartOfHeading,
    StartOfText,
    EndOfText,
    EndOfTransmission,
    Enquiry,
    Acknowledge,
    Alert,
    Backspace,
    CharacterTabulation,
    LineFeed,
    LineTabulation,
    FormFeed,
    CarriageReturn,
    ShiftOut,
    ShiftIn,
    DataLinkEscape,
    DeviceControlOne,
    DeviceControlTwo,
    DeviceControlThree,
    DeviceControlFour,
    NegativeAcknowledge,
    SynchronousIdle,
    EndOfTransmissionBlock,
    Cancel,
    EndOfMedium,
    Substitute,
    Escape,
    FileSeparator,
    GroupSeparator,
    RecordSeparator,
    UnitSeparator,
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