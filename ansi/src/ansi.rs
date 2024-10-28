use crate::vt::{Action, VTParser};

macro_rules! parse_color {
    ($iter:expr, $color_type:ident) => {
        match $iter.next() {
            Some(5) => match $iter.next() {
                Some(color) => Some($color_type(Color::Indexed(color as u8))),
                None => None,
            },
            Some(2) => match ($iter.next(), $iter.next(), $iter.next()) {
                (Some(r), Some(g), Some(b)) => {
                    Some($color_type(Color::RGB(r as u8, g as u8, b as u8)))
                }
                _ => None,
            },
            _ => None,
        }
    };
}

#[derive(Debug, PartialEq)]
pub enum Color {
    Default,
    Indexed(u8),
    RGB(u8, u8, u8),
}

#[derive(Debug, PartialEq)]
pub enum AnsiCommand {
    Print(char),
    CursorUp(u16),
    CursorDown(u16),
    CursorForward(u16),
    CursorBackward(u16),
    CursorPosition(u16, u16),
    EraseInDisplay(u8),
    EraseInLine(u8),
    Sgr(Option<Sgr>),
}

/// this shit is not exhaustive
#[derive(Debug, PartialEq)]
pub enum Sgr {
    Reset,
    Bold,
    Faint,
    Italic,
    Underlined(bool),
    Blink(BlinkInterval),
    Inverted(bool),
    Conceal(bool),
    CrossedOut(bool),
    PrimaryFont,
    AlternativeFont(u8),
    Fraktur,
    DoublyUnderlined,
    Regular,
    NeitherItalicNorBlackletter,
    ProportionalSpacing(bool),
    ForegroundColor(Color),
    BackgroundColor(Color),
    Framed,
    Encircled,
    Overlined(bool),
    NeitherFramedNorEncircled,
    UnderlineColor(Color),
}

#[derive(Debug, PartialEq)]
pub enum BlinkInterval {
    Slow,
    Rapid,
    Static,
}

#[derive(Default)]
pub struct AnsiParser {
    vt_parser: VTParser,
}

impl AnsiParser {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn parse<F>(&mut self, data: &[u8], mut callback: F)
    where
        F: FnMut(AnsiCommand),
    {
        for &byte in data {
            let action = self.vt_parser.parse_byte(byte);
            dbg!(&action);
            if let Some(command) = self.interpret_action(action) {
                callback(command);
            }
        }
    }

    fn interpret_action(&self, action: Action) -> Option<AnsiCommand> {
        use AnsiCommand::*;

        match action {
            Action::Print(c) => Some(Print(c)),
            Action::CsiDispatch(byte, params) => {
                let p1 = params.first().copied().flatten().unwrap_or(1) as u16;

                match byte {
                    b'A' => Some(CursorUp(p1)),
                    b'B' => Some(CursorDown(p1)),
                    b'C' => Some(CursorForward(p1)),
                    b'D' => Some(CursorBackward(p1)),
                    b'H' | b'f' => {
                        let row = p1;
                        let col = params.get(1).copied().flatten().unwrap_or(1) as u16;

                        Some(CursorPosition(row, col))
                    }
                    b'J' => Some(EraseInDisplay(p1 as u8)),
                    b'K' => Some(EraseInLine(p1 as u8)),
                    b'm' => Some(AnsiCommand::Sgr(self.interpret_sgr(&params))),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    fn interpret_sgr(&self, params: &[Option<i32>]) -> Option<Sgr> {
        use Sgr::*;

        if params.is_empty() {
            return Some(Reset);
        }

        let mut iter = params.iter().flatten().copied();
        while let Some(code) = iter.next() {
            dbg!(code);
            match code {
                0 => return Some(Reset),
                1 => return Some(Bold),
                2 => return Some(Faint),
                3 => return Some(Italic),
                4 => return Some(Underlined(true)),
                5 => return Some(Blink(BlinkInterval::Slow)),
                6 => return Some(Blink(BlinkInterval::Rapid)),
                7 => return Some(Inverted(true)),
                8 => return Some(Conceal(true)),
                9 => return Some(CrossedOut(true)),
                10 => return Some(PrimaryFont),
                11..=19 => return Some(AlternativeFont((code - 10) as u8)),
                20 => return Some(Fraktur),
                21 => return Some(DoublyUnderlined),
                22 => return Some(Regular),
                23 => return Some(NeitherItalicNorBlackletter),
                24 => return Some(Underlined(false)),
                25 => return Some(Blink(BlinkInterval::Static)),
                26 => return Some(ProportionalSpacing(true)),
                27 => return Some(Inverted(false)),
                28 => return Some(Conceal(false)),
                29 => return Some(CrossedOut(false)),
                30..=37 => return Some(ForegroundColor(Color::Indexed((code - 30) as u8))),
                38 => return parse_color!(iter, ForegroundColor),
                39 => return Some(ForegroundColor(Color::Default)),
                40..=47 => return Some(BackgroundColor(Color::Indexed((code - 40) as u8))),
                48 => return parse_color!(iter, BackgroundColor),
                49 => return Some(BackgroundColor(Color::Default)),
                50 => return Some(ProportionalSpacing(false)),
                51 => return Some(Framed),
                52 => return Some(Encircled),
                53 => return Some(Overlined(true)),
                54 => return Some(NeitherFramedNorEncircled),
                55 => return Some(Overlined(false)),
                58 => return parse_color!(iter, UnderlineColor),
                59 => return Some(UnderlineColor(Color::Default)),
                _ => {}
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use AnsiCommand::*;

    #[test]
    fn simple_text() {
        let mut parser = AnsiParser::new();
        let mut output = vec![];

        parser.parse(b"foobar", |cmd| output.push(cmd));

        assert_eq!(
            output,
            vec![
                Print('f'),
                Print('o'),
                Print('o'),
                Print('b'),
                Print('a'),
                Print('r'),
            ]
        );
    }

    #[test]
    fn cursor_movement() {
        let mut parser = AnsiParser::new();
        let mut output = vec![];

        parser.parse(
            b"\
            \x1B[1A\
            \x1B[2B\
            \x1B[3C\
            \x1B[4D",
            |cmd| output.push(cmd),
        );

        assert_eq!(
            output,
            vec![
                CursorUp(1),
                CursorDown(2),
                CursorForward(3),
                CursorBackward(4)
            ]
        );
    }

    #[test]
    fn colors() {
        let mut parser = AnsiParser::new();
        let mut output = vec![];

        parser.parse(
            b"\
            \x1B[31m\
            \x1B[38;5;123m\
            \x1B[38;2;1;12;123m\
            \x1B[39m\
            \x1B[41m\
            \x1B[48;5;123m\
            \x1B[48;2;1;12;123m\
            \x1B[49m\
            ",
            |cmd| output.push(cmd),
        );

        assert_eq!(
            output,
            vec![
                Sgr(Some(crate::ansi::Sgr::ForegroundColor(Color::Indexed(1)))),
                Sgr(Some(crate::ansi::Sgr::ForegroundColor(Color::Indexed(123)))),
                Sgr(Some(crate::ansi::Sgr::ForegroundColor(Color::RGB(
                    1, 12, 123
                )))),
                Sgr(Some(crate::ansi::Sgr::ForegroundColor(Color::Default))),
                Sgr(Some(crate::ansi::Sgr::BackgroundColor(Color::Indexed(1)))),
                Sgr(Some(crate::ansi::Sgr::BackgroundColor(Color::Indexed(123)))),
                Sgr(Some(crate::ansi::Sgr::BackgroundColor(Color::RGB(
                    1, 12, 123
                )))),
                Sgr(Some(crate::ansi::Sgr::BackgroundColor(Color::Default))),
            ]
        );
    }
}
