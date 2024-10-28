use crate::vt::{Action, VTParser};

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
    SetForegroundColor(Color),
    SetBackgroundColor(Color),
    Reset,
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
                    b'm' => self.interpret_sgr(&params),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    fn interpret_sgr(&self, params: &[Option<i32>]) -> Option<AnsiCommand> {
        use AnsiCommand::*;

        if params.is_empty() {
            return Some(Reset);
        }

        let mut iter = params.iter().flatten().copied();
        while let Some(code) = iter.next() {
            dbg!(code);
            match code {
                0 => return Some(Reset),
                30..=37 => return Some(SetForegroundColor(Color::Indexed((code - 30) as u8))),
                38 => match iter.next() {
                    Some(5) => {
                        if let Some(color) = iter.next() {
                            return Some(SetForegroundColor(Color::Indexed(color as u8)));
                        }
                    }
                    Some(2) => {
                        if let (Some(r), Some(g), Some(b)) = (iter.next(), iter.next(), iter.next())
                        {
                            return Some(SetForegroundColor(Color::RGB(r as u8, g as u8, b as u8)));
                        }
                    }
                    _ => {}
                },
                39 => return Some(SetForegroundColor(Color::Default)),
                40..=47 => return Some(SetBackgroundColor(Color::Indexed((code - 40) as u8))),
                48 => match iter.next() {
                    Some(5) => {
                        if let Some(color) = iter.next() {
                            return Some(SetBackgroundColor(Color::Indexed(color as u8)));
                        }
                    }
                    Some(2) => {
                        if let (Some(r), Some(g), Some(b)) = (iter.next(), iter.next(), iter.next())
                        {
                            return Some(SetBackgroundColor(Color::RGB(r as u8, g as u8, b as u8)));
                        }
                    }
                    _ => {}
                },
                49 => return Some(SetBackgroundColor(Color::Default)),
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
                SetForegroundColor(Color::Indexed(1)),
                SetForegroundColor(Color::Indexed(123)),
                SetForegroundColor(Color::RGB(1, 12, 123)),
                SetForegroundColor(Color::Default),
                SetBackgroundColor(Color::Indexed(1)),
                SetBackgroundColor(Color::Indexed(123)),
                SetBackgroundColor(Color::RGB(1, 12, 123)),
                SetBackgroundColor(Color::Default),
            ]
        );
    }
}
