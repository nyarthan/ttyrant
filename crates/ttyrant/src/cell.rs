#![allow(dead_code)]

use std::ops::Deref;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Buffer {
    lines: Vec<Line>,
    width: usize,
    height: usize,
}

impl Buffer {
    pub fn new(width: usize, height: usize) -> Self {
        let mut lines = Vec::with_capacity(height);
        for _ in 0..height {
            lines.push(Line::new(width as u16));
        }
        Self {
            lines,
            width,
            height,
        }
    }

    pub fn write_str(&mut self, x: usize, y: usize, s: &str, attrs: CellAttributes) {
        if y >= self.height {
            return;
        }

        let line = &mut self.lines[y];
        for (i, ch) in s.chars().enumerate() {
            if x + i >= self.width {
                break;
            }
            line.set((x + i) as u16, Cell::new(ch, attrs))
        }
    }
}

const INLINE_CELLS: usize = 8;

#[derive(Debug, Clone)]
pub struct Line {
    inline_cells: [(u16, Cell); INLINE_CELLS],
    inline_count: u8,
    overflow: Option<Box<Vec<(u16, Cell)>>>,
    attributes: CellAttributes,
    width: u16,
}

impl Line {
    pub fn new(width: u16) -> Self {
        Self {
            inline_cells: Default::default(),
            inline_count: 0,
            overflow: None,
            attributes: CellAttributes::default(),
            width,
        }
    }

    pub fn get(&self, x: u16) -> Cell {
        dbg!("get with lc", self.inline_count);
        if x >= self.width {
            return Cell::default();
        }

        for i in 0..self.inline_count as usize {
            dbg!(self.inline_cells[i]);
            if self.inline_cells[i].0 == x {
                return self.inline_cells[i].1;
            }
        }

        if let Some(overflow) = &self.overflow {
            if let Ok(i) = overflow.binary_search_by_key(&x, |(pos, _)| *pos) {
                return overflow[i].1;
            }
        }

        Cell::default()
    }

    pub fn set(&mut self, x: u16, cell: Cell) {
        if x >= self.width {
            return;
        }

        let cell_is_default = cell.is_default();

        for i in 0..self.inline_count as usize {
            if self.inline_cells[i].0 == x {
                if cell_is_default {
                    self.inline_cells
                        .copy_within(i + 1..self.inline_count as usize, i);
                    self.inline_count -= 1;
                } else {
                    self.inline_cells[i].1 = cell;
                }
            }
            return;
        }

        if cell_is_default {
            if let Some(overflow) = &mut self.overflow {
                if let Ok(i) = overflow.binary_search_by_key(&x, |(pos, _)| *pos) {
                    overflow.remove(i);
                }
            };
            return;
        }

        if self.inline_count < INLINE_CELLS as u8 {
            let i = self.find_insert_position(x);
            self.inline_cells
                .copy_within(i..self.inline_count as usize, i + 1);
            self.inline_cells[i] = (x, cell);
            self.inline_count += 1;
            dbg!("set with count", self.inline_count);
            return;
        }

        let overflow = self.overflow.get_or_insert_with(|| Box::new(Vec::new()));
        match overflow.binary_search_by_key(&x, |(pos, _)| *pos) {
            Ok(i) => overflow[i].1 = cell,
            Err(i) => overflow.insert(i, (x, cell)),
        };
    }

    fn find_insert_position(&self, x: u16) -> usize {
        for i in 0..self.inline_count as usize {
            if self.inline_cells[i].0 > x {
                return i;
            }
        }
        self.inline_count as usize
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Cell {
    pub ch: char,
    pub attrs: CellAttributes,
    _padding: [u8; 3],
    pub fg: Color,
    pub bg: Color,
}

impl Cell {
    pub fn new(ch: char, attrs: CellAttributes) -> Self {
        Self {
            ch,
            attrs,
            ..Default::default()
        }
    }
    pub fn is_default(self) -> bool {
        self == Self::default()
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            ch: ' ',
            attrs: CellAttributes::default(),
            _padding: [0x00, 0x00, 0x00],
            fg: Color::indexed(0),
            bg: Color::indexed(0),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct CellAttributes {}

impl Default for CellAttributes {
    fn default() -> Self {
        Self {}
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color(u32);

impl Color {
    const R_SHIFT: u32 = 16;
    const G_SHIFT: u32 = 8;
    const B_SHIFT: u32 = 0;

    const R_MASK: u32 = 0xFF << Self::R_SHIFT;
    const G_MASK: u32 = 0xFF << Self::G_SHIFT;
    const B_MASK: u32 = 0xFF << Self::B_SHIFT;

    const INDEX_FLAG: u32 = 1 << 31;

    #[inline]
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self(
            ((r as u32) << Self::R_SHIFT)
                | ((g as u32) << Self::G_SHIFT)
                | ((b as u32) << Self::B_SHIFT),
        )
    }

    #[inline]
    pub const fn indexed(index: u8) -> Self {
        Self(Self::INDEX_FLAG | (index as u32))
    }

    #[inline]
    pub fn r(&self) -> u8 {
        ((**self & Self::R_MASK) >> Self::R_SHIFT) as u8
    }

    #[inline]
    pub fn g(&self) -> u8 {
        ((**self & Self::G_MASK) >> Self::G_SHIFT) as u8
    }

    #[inline]
    pub fn b(&self) -> u8 {
        ((**self & Self::B_MASK) >> Self::B_SHIFT) as u8
    }

    #[inline]
    pub fn is_indexed(&self) -> bool {
        **self & Self::INDEX_FLAG != 0
    }

    #[inline]
    pub fn index(&self) -> Option<u8> {
        if self.is_indexed() {
            Some(**self as u8)
        } else {
            None
        }
    }
}

impl Deref for Color {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_single_char_to_cell() {
        let mut buffer = Buffer::new(10, 10);
        buffer.write_str(0, 0, "C", CellAttributes::default());
        let char = buffer.lines.first().unwrap().get(0).ch;

        assert_eq!(char, 'C');
    }

    #[test]
    fn write_string_to_cells() {
        let mut buffer = Buffer::new(10, 10);
        buffer.write_str(0, 0, "CCC", CellAttributes::default());
        let char1 = buffer.lines.first().unwrap().get(0).ch;
        let char2 = buffer.lines.first().unwrap().get(1).ch;
        let char3 = buffer.lines.first().unwrap().get(2).ch;

        assert_eq!(char1, 'C');
        assert_eq!(char2, 'C');
        assert_eq!(char3, 'C');
    }
}
