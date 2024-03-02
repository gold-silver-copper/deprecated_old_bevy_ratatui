//! This module provides the `BracketBackend` implementation for the [`Backend`] trait.
//! It is used in the integration tests to verify the correctness of the library.

use bracket_lib::{color::RGB as BracketColor, prelude::*};

use std::{
    fmt::{Display, Write},
    io,
};

use ratatui::{
    backend::{Backend, ClearType, WindowSize},
    buffer::{Buffer, Cell},
    layout::{Rect, Size},
    style::{Color as RatColor, Modifier},
};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BracketBackend {
    width: u16,
    buffer: Buffer,
    height: u16,
    cursor: bool,
    pos: (u16, u16),
    pub bracket_term: BTerm,
}

impl BracketBackend {
    /// Creates a new BracketBackend with the specified width and height.
    pub fn new(bterm: BTerm) -> BracketBackend {
        let t_size = bterm.get_char_size();
        BracketBackend {
            width: t_size.0 as u16,
            height: t_size.1 as u16,
            buffer: Buffer::empty(Rect::new(0, 0, t_size.0 as u16, t_size.1 as u16)),
            cursor: false,
            pos: (0, 0),
            bracket_term: bterm,
        }
    }

    /// Returns a reference to the internal buffer of the BracketBackend.
    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    /// Resizes the BracketBackend to the specified width and height.
    pub fn resize(&mut self, width: u16, height: u16) {
        self.buffer.resize(Rect::new(0, 0, width, height));
        self.width = width;
        self.height = height;
    }

    fn update_from_buffer(&mut self) {
        let buff = self.buffer().clone();

        for x in 0..self.width {
            for y in 0..self.height {
                draw_from_xyc(&mut self.bracket_term, x, y, buff.get(x, y));
            }
        }
    }
}

trait FromRatColor<RatColor> {
    fn from_rat_color(color: RatColor, fg: bool) -> BracketColor;
}

impl FromRatColor<RatColor> for BracketColor {
    fn from_rat_color(color: RatColor, fg: bool) -> Self {
        match color {
            RatColor::Reset => {
                if fg {
                    BracketColor::named(WHITE)
                } else {
                    BracketColor::named(DARKGRAY)
                }
            }
            RatColor::Black => BracketColor::named(BLACK),
            RatColor::Red => BracketColor::named(DARKRED),
            RatColor::Green => BracketColor::named(DARKGREEN),
            RatColor::Yellow => BracketColor::named(DARKGOLDENROD),
            RatColor::Blue => BracketColor::named(DARKBLUE),
            RatColor::Magenta => BracketColor::named(DARKMAGENTA),
            RatColor::Cyan => BracketColor::named(DARK_CYAN),
            RatColor::Gray => BracketColor::named(GRAY),
            RatColor::DarkGray => BracketColor::named(DARKGRAY),
            RatColor::LightRed => BracketColor::named(RED),
            RatColor::LightGreen => BracketColor::named(GREEN),
            RatColor::LightBlue => BracketColor::named(BLUE),
            RatColor::LightYellow => BracketColor::named(BISQUE),
            RatColor::LightMagenta => BracketColor::named(MAGENTA2),
            RatColor::LightCyan => BracketColor::named(AQUAMARINE),
            RatColor::White => BracketColor::named(WHITE),
            RatColor::Indexed(i) => BracketColor::from_u8(i, i, i),
            RatColor::Rgb(r, g, b) => BracketColor::from_u8(r, g, b),
        }
    }
}

fn draw_from_xyc(b_term: &mut BTerm, x: u16, y: u16, c: &Cell) {
    let fg = BracketColor::from_rat_color(c.fg, true);
    let bg = BracketColor::from_rat_color(c.bg, false);

    let mut proper_value = c.symbol().to_string();

    if c.modifier.intersects(Modifier::UNDERLINED) {
        proper_value = format!("{}{}", proper_value, '\u{0332}');
    }

    if c.modifier.intersects(Modifier::CROSSED_OUT) {
        proper_value = format!("{}{}", proper_value, '\u{0336}');
    }

    b_term.print_color(x, y, fg, bg, proper_value.as_str());
}

impl Backend for BracketBackend {
    fn draw<'a, I>(&mut self, content: I) -> Result<(), io::Error>
    where
        I: Iterator<Item = (u16, u16, &'a Cell)>,
    {
        self.bracket_term
            .set_translation_mode(0, CharacterTranslationMode::Unicode);

        for (x, y, c) in content {
            // self.bracket_term.cls();

            // draw_from_xyc(&mut self.bracket_term,x,y,c);

            let cell = self.buffer.get_mut(x, y);
            *cell = c.clone();
        }
        self.update_from_buffer();
        Ok(())
    }

    fn hide_cursor(&mut self) -> Result<(), io::Error> {
        self.cursor = false;
        Ok(())
    }

    fn show_cursor(&mut self) -> Result<(), io::Error> {
        self.cursor = true;
        Ok(())
    }

    fn get_cursor(&mut self) -> Result<(u16, u16), io::Error> {
        Ok(self.pos)
    }

    fn set_cursor(&mut self, x: u16, y: u16) -> Result<(), io::Error> {
        self.pos = (x, y);
        Ok(())
    }

    fn clear(&mut self) -> Result<(), io::Error> {
        self.buffer.reset();
        self.bracket_term.cls();
        Ok(())
    }

    fn clear_region(&mut self, clear_type: ClearType) -> io::Result<()> {
        match clear_type {
            ClearType::All => self.clear()?,
            ClearType::AfterCursor => {
                let index = self.buffer.index_of(self.pos.0, self.pos.1) + 1;
                self.buffer.content[index..].fill(Cell::default());
            }
            ClearType::BeforeCursor => {
                let index = self.buffer.index_of(self.pos.0, self.pos.1);
                self.buffer.content[..index].fill(Cell::default());
            }
            ClearType::CurrentLine => {
                let line_start_index = self.buffer.index_of(0, self.pos.1);
                let line_end_index = self.buffer.index_of(self.width - 1, self.pos.1);
                self.buffer.content[line_start_index..=line_end_index].fill(Cell::default());
            }
            ClearType::UntilNewLine => {
                let index = self.buffer.index_of(self.pos.0, self.pos.1);
                let line_end_index = self.buffer.index_of(self.width - 1, self.pos.1);
                self.buffer.content[index..=line_end_index].fill(Cell::default());
            }
        }
        self.update_from_buffer();
        Ok(())
    }

    fn append_lines(&mut self, n: u16) -> io::Result<()> {
        let (cur_x, cur_y) = self.get_cursor()?;

        // the next column ensuring that we don't go past the last column
        let new_cursor_x = cur_x.saturating_add(1).min(self.width.saturating_sub(1));

        let max_y = self.height.saturating_sub(1);
        let lines_after_cursor = max_y.saturating_sub(cur_y);
        if n > lines_after_cursor {
            let rotate_by = n.saturating_sub(lines_after_cursor).min(max_y);

            if rotate_by == self.height - 1 {
                self.clear()?;
            }

            self.set_cursor(0, rotate_by)?;
            self.clear_region(ClearType::BeforeCursor)?;
            self.buffer
                .content
                .rotate_left((self.width * rotate_by).into());
        }

        let new_cursor_y = cur_y.saturating_add(n).min(max_y);
        self.set_cursor(new_cursor_x, new_cursor_y)?;
        self.update_from_buffer();

        Ok(())
    }

    fn size(&self) -> Result<Rect, io::Error> {
        Ok(Rect::new(0, 0, self.width, self.height))
    }

    fn window_size(&mut self) -> Result<WindowSize, io::Error> {
        // Some arbitrary window pixel size, probably doesn't need much testing.
        static WINDOW_PIXEL_SIZE: Size = Size {
            width: 640,
            height: 480,
        };
        Ok(WindowSize {
            columns_rows: (self.width, self.height).into(),
            pixels: WINDOW_PIXEL_SIZE,
        })
    }

    fn flush(&mut self) -> Result<(), io::Error> {
        Ok(())
    }
}
