use std::mem::MaybeUninit;

#[derive(Clone, Copy)]
pub enum State {
    Ground,
    Escape,
    EscapeIntermediate,
    CsiEntry,
    CsiParam,
    CsiIntermediate,
    CsiIgnore,
    OscString,
}

#[derive(Debug, PartialEq)]
pub enum Action {
    Print(char),
    Execute(u8),
    Clear,
    CollectParam(u8),
    Hook(Vec<i32>, Vec<u8>),
    Put(u8),
    Unhook,
    OscStart,
    OscPut(u8),
    OscEnd,
    CsiDispatch(u8, Vec<Option<i32>>),
    EscDispatch(u8),
    None,
}

pub struct Params {
    data: [MaybeUninit<Option<i32>>; 16],
    len: usize,
    current: Option<i32>,
    has_current: bool,
}

impl Params {
    fn push_digit(&mut self, digit: u8) {
        let digit = (digit - b'0') as i32;
        self.current = Some(self.current.unwrap_or(0) * 10 + digit);
        self.has_current = true;
    }

    fn finish_param(&mut self) {
        if self.len < self.data.len() {
            self.data[self.len].write(if self.has_current { self.current } else { None });
            self.len += 1;
        }
        self.current = None;
        self.has_current = false;
    }

    fn as_slice(&self) -> &[Option<i32>] {
        unsafe { std::slice::from_raw_parts(self.data.as_ptr() as *const Option<i32>, self.len) }
    }
}

impl Default for Params {
    fn default() -> Self {
        Self {
            data: unsafe { MaybeUninit::uninit().assume_init() },
            len: 0,
            current: None,
            has_current: false,
        }
    }
}

pub struct VTParser {
    state: State,
    params: Params,
    intermediates: Vec<u8>,
}

impl Default for VTParser {
    fn default() -> Self {
        Self {
            state: State::Ground,
            params: Params::default(),
            intermediates: Vec::with_capacity(4),
        }
    }
}

impl VTParser {
    pub fn parse_byte(&mut self, byte: u8) -> Action {
        use Action::*;
        use State::*;

        match (self.state, byte) {
            (Ground, 0x1B) => {
                self.state = Escape;
                None
            }
            (Ground, 0x20..=0x7F) => Print(byte as char),
            (Ground, 0x00..=0x1F) => Execute(byte),

            (Escape, 0x5B) => {
                self.state = CsiEntry;
                self.params = Params::default();
                None
            }
            (Escape, 0x20..=0x2F) => {
                self.state = EscapeIntermediate;
                self.intermediates.push(byte);
                None
            }
            (Escape, 0x30..=0x7E) => {
                self.state = Ground;
                EscDispatch(byte)
            }

            (CsiEntry, 0x30..=0x39) => {
                self.state = CsiParam;
                self.params.push_digit(byte);
                None
            }
            (CsiEntry, 0x3B) => {
                self.state = CsiParam;
                self.params.finish_param();
                None
            }
            (CsiEntry, 0x40..=0x7E) => {
                let params = self.params.as_slice().to_vec();
                self.state = Ground;
                CsiDispatch(byte, params)
            }

            (CsiParam, 0x30..=0x39) => {
                self.params.push_digit(byte);
                None
            }
            (CsiParam, 0x3B) => {
                self.params.finish_param();
                None
            }
            (CsiParam, 0x40..=0x7E) => {
                self.params.finish_param();
                let params = self.params.as_slice().to_vec();
                self.state = Ground;
                CsiDispatch(byte, params)
            }

            _ => {
                self.state = Ground;
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Action::*;

    fn parse_bytes(bytes: &[u8]) -> Vec<Action> {
        let mut parser = VTParser::default();
        let mut actions = Vec::new();

        for &byte in bytes {
            let action = parser.parse_byte(byte);
            if action != Action::None {
                actions.push(action);
            }
        }

        actions
    }

    #[test]
    fn simple_text() {
        let actions = parse_bytes(b"Hello");
        assert_eq!(
            actions,
            vec![Print('H'), Print('e'), Print('l'), Print('l'), Print('o'),]
        );
    }

    #[test]
    fn control_characters() {
        let actions = parse_bytes(b"Hello\x07World\x08");
        assert_eq!(
            actions,
            vec![
                Print('H'),
                Print('e'),
                Print('l'),
                Print('l'),
                Print('o'),
                Execute(0x07), // Bell
                Print('W'),
                Print('o'),
                Print('r'),
                Print('l'),
                Print('d'),
                Execute(0x08), // Backspace
            ]
        );
    }

    #[test]
    fn cursor_movement() {
        let tests = vec![
            (
                b"\x1B[A".to_vec(),
                // FIXME: should be `vec![None]
                vec![CsiDispatch(b'A', vec![])],
                "cursor up default",
            ),
            (
                b"\x1B[5B".to_vec(),
                vec![CsiDispatch(b'B', vec![Some(5)])],
                "cursor down with value",
            ),
            (
                b"\x1B[;C".to_vec(),
                vec![CsiDispatch(b'C', vec![Option::None, Option::None])],
                "cursor right empty param",
            ),
            (
                b"\x1B[10;20H".to_vec(),
                vec![CsiDispatch(b'H', vec![Some(10), Some(20)])],
                "cursor position",
            ),
        ];

        for (input, expected, message) in tests {
            assert_eq!(parse_bytes(&input), expected, "{}", message);
        }
    }

    #[test]
    fn sgr_colors() {
        // Test SGR (Select Graphic Rendition) sequences
        let tests = vec![
            (
                b"\x1B[31m".to_vec(),
                vec![CsiDispatch(b'm', vec![Some(31)])],
                "basic foreground color",
            ),
            (
                b"\x1B[46m".to_vec(),
                vec![CsiDispatch(b'm', vec![Some(46)])],
                "basic background color",
            ),
            (
                b"\x1B[38;5;123m".to_vec(),
                vec![CsiDispatch(b'm', vec![Some(38), Some(5), Some(123)])],
                "256 color foreground",
            ),
            (
                b"\x1B[48;2;255;128;0m".to_vec(),
                vec![CsiDispatch(
                    b'm',
                    vec![Some(48), Some(2), Some(255), Some(128), Some(0)],
                )],
                "RGB background color",
            ),
        ];

        for (input, expected, message) in tests {
            assert_eq!(parse_bytes(&input), expected, "{}", message);
        }
    }

    // #[test]
    // fn dec_private_modes() {
    //     let tests = vec![
    //         (
    //             b"\x1B[?25h".to_vec(),
    //             vec![Action::DecSetMode(PrivateMode::ShowCursor)],
    //             "show cursor"
    //         ),
    //         (
    //             b"\x1B[?25l".to_vec(),
    //             vec![Action::DecResetMode(PrivateMode::ShowCursor)],
    //             "hide cursor"
    //         ),
    //         (
    //             b"\x1B[?1h".to_vec(),
    //             vec![Action::DecSetMode(PrivateMode::CursorKeys)],
    //             "set cursor keys"
    //         ),
    //         (
    //             b"\x1B[?1049h".to_vec(),
    //             vec![Action::DecSetMode(PrivateMode::Other(1049))],
    //             "alternate screen buffer"
    //         ),
    //     ];
    //
    //     for (input, expected, message) in tests {
    //         assert_eq!(parse_bytes(&input), expected, "{}", message);
    //     }
    // }

    #[test]
    fn partial_sequences() {
        let mut parser = VTParser::default();
        let mut actions = Vec::new();

        // Feed sequence byte by byte
        for &byte in b"\x1B[31m" {
            let action = parser.parse_byte(byte);
            if action != None {
                actions.push(action);
            }
        }

        assert_eq!(actions, vec![CsiDispatch(b'm', vec![Some(31)])]);
    }

    #[test]
    fn invalid_sequences() {
        let tests = vec![
            (
                b"\x1B[1;2;3x".to_vec(), // Invalid final byte
                "invalid final byte",
            ),
            (
                b"\x1B[1;\x1B[2m".to_vec(), // Interrupted sequence
                "interrupted sequence",
            ),
            (
                b"\x1B[a1m".to_vec(), // Invalid parameter
                "invalid parameter",
            ),
        ];

        for (input, message) in tests {
            let actions = parse_bytes(&input);
            // Should not panic and should return to ground state
            assert!(actions.len() > 0, "{}", message);
        }
    }

    #[test]
    fn mixed_content() {
        let input = b"Hello\x1B[31mWorld\x1B[0m!";
        let actions = parse_bytes(input);

        let expected = vec![
            Print('H'),
            Print('e'),
            Print('l'),
            Print('l'),
            Print('o'),
            CsiDispatch(b'm', vec![Some(31)]),
            Print('W'),
            Print('o'),
            Print('r'),
            Print('l'),
            Print('d'),
            CsiDispatch(b'm', vec![Some(0)]),
            Print('!'),
        ];

        assert_eq!(actions, expected);
    }

    #[test]
    fn parameter_limits() {
        // Test parameter number limits
        let mut large_params = Vec::new();
        large_params.extend(b"\x1B[");
        for i in 1..=20 {
            large_params.extend(i.to_string().bytes());
            large_params.push(b';');
        }
        large_params.push(b'm');

        let actions = parse_bytes(&large_params);
        assert_eq!(actions.len(), 1, "should handle many parameters");

        if let CsiDispatch(b'm', params) = &actions[0] {
            assert!(params.len() <= 16, "should limit number of parameters");
        } else {
            panic!("unexpected action");
        }
    }

    #[test]
    fn parameter_values() {
        let tests = vec![
            (
                b"\x1B[123456789m".to_vec(),
                vec![CsiDispatch(b'm', vec![Some(123456789)])],
                "large parameter value",
            ),
            (
                b"\x1B[0m".to_vec(),
                vec![CsiDispatch(b'm', vec![Some(0)])],
                "zero parameter",
            ),
            (
                b"\x1B[m".to_vec(),
                vec![CsiDispatch(b'm', vec![])],
                "no parameter",
            ),
        ];

        for (input, expected, message) in tests {
            assert_eq!(parse_bytes(&input), expected, "{}", message);
        }
    }

    #[test]
    fn stress() {
        // Create a large input with mixed content
        let mut large_input = Vec::new();
        for _ in 0..1000 {
            large_input.extend(b"Hello\x1B[31mWorld\x1B[0m!");
        }

        // Should not panic and should process all input
        let actions = parse_bytes(&large_input);
        assert!(actions.len() > 0, "should process large input");
    }
}
