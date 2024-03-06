//! This module provides the `BevyBackend` implementation for the [`Backend`] trait.
//! It is used in the integration tests to verify the correctness of the library.

use std::io;

use bevy::{prelude::*, utils::HashMap};

use ratatui::{
    backend::{Backend, ClearType, WindowSize},
    buffer::{Buffer, Cell},
    layout::{Rect, Size},
};

///RATATUI SPECIFIC STUFF STARTS HERE
///
///
/// Bevy Backend is created either with default() which uses the built in Bevy font which is VERY
/// bad and not recommended, or with new() which takes the height,width font size, and path strings
/// to fonts to be used for the normal, bold, italic, and bold italic text variants.

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BevyBackend {
    pub height: u16,
    pub width: u16,
    pub term_font_size: u16,
    /// HashMap with position tuple index and UI Entity id (cell) in that position
    pub entity_map: HashMap<(u16, u16), Entity>,
    pub buffer: Buffer,

    pub vcupdate: Vec<(u16, u16, Cell)>,
    pub cursor_ref: Entity,

    pub cursor: bool,
    pub cursor_pos: (u16, u16),
    pub bevy_initialized: bool,
    pub normal_font_path: Option<String>,
    pub italic_font_path: Option<String>,
    pub bold_font_path: Option<String>,
    pub italicbold_font_path: Option<String>,
    pub normal_handle: Handle<Font>,
    pub italic_handle: Handle<Font>,
    pub bold_handle: Handle<Font>,
    pub italicbold_handle: Handle<Font>,
    pub manual_window_sizing: bool,
}

impl Default for BevyBackend {
    fn default() -> Self {
        BevyBackend {
            height: 25,
            width: 40,
            term_font_size: 30,
            entity_map: HashMap::new(),
            buffer: Buffer::empty(Rect::new(0, 0, 40, 25)),

            vcupdate: Vec::default(),
            cursor: false,
            cursor_pos: (0, 0),
            bevy_initialized: false,
            cursor_ref: Entity::PLACEHOLDER,

            normal_font_path: None,
            italic_font_path: None,
            bold_font_path: None,
            italicbold_font_path: None,
            normal_handle: Handle::weak_from_u128(101),
            italic_handle: Handle::weak_from_u128(101),
            bold_handle: Handle::weak_from_u128(101),
            italicbold_handle: Handle::weak_from_u128(101),
            manual_window_sizing: false,
        }
    }
}

impl BevyBackend {
    /// Creates a new BevyBackend with the specified width and height.
    pub fn new(
        width: u16,
        height: u16,
        font_size: u16,
        normal_font_path: &str,
        italic_font_path: &str,
        bold_font_path: &str,
        italicbold_font_path: &str,
    ) -> BevyBackend {
        BevyBackend {
            height: height,
            width: width,
            term_font_size: font_size,
            entity_map: HashMap::new(),
            buffer: Buffer::empty(Rect::new(0, 0, width, height)),

            vcupdate: Vec::default(),
            cursor: false,
            cursor_pos: (0, 0),
            cursor_ref: Entity::PLACEHOLDER,

            bevy_initialized: false,
            normal_font_path: Some(normal_font_path.to_string()),
            italic_font_path: Some(italic_font_path.to_string()),
            bold_font_path: Some(bold_font_path.to_string()),
            italicbold_font_path: Some(italicbold_font_path.to_string()),
            normal_handle: Handle::weak_from_u128(101),
            italic_handle: Handle::weak_from_u128(101),
            bold_handle: Handle::weak_from_u128(101),
            italicbold_handle: Handle::weak_from_u128(101),
            manual_window_sizing: false,
        }
    }

    /// Resizes the BevyBackend to the specified width and height.
    pub fn resize(&mut self, width: u16, height: u16) {
        self.buffer.resize(Rect::new(0, 0, width, height));
        self.width = width;
        self.height = height;
    }

    /// Resizes the BevyBackend to the specified width and height.
    pub fn manual_window_sizing(&mut self, value: bool) {
        self.manual_window_sizing = value;
    }
}

impl Backend for BevyBackend {
    fn draw<'a, I>(&mut self, content: I) -> Result<(), io::Error>
    where
        I: Iterator<Item = (u16, u16, &'a Cell)>,
    {
        for (x, y, c) in content {
            if !c.skip {self.vcupdate.push((x, y, c.clone()));
                let cell = self.buffer.get_mut(x, y);
                *cell = c.clone();
    
                // println!("{} {}", x, y);
                //  println!("{:?}", c);}
            

          
        }}
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
        Ok(self.cursor_pos)
    }

    fn set_cursor(&mut self, x: u16, y: u16) -> Result<(), io::Error> {
        self.cursor_pos = (x, y);
        Ok(())
    }

    fn clear(&mut self) -> Result<(), io::Error> {
        self.buffer.reset();
        Ok(())
    }

    fn clear_region(&mut self, clear_type: ClearType) -> io::Result<()> {
        match clear_type {
            ClearType::All => self.clear()?,
            ClearType::AfterCursor => {
                let index = self.buffer.index_of(self.cursor_pos.0, self.cursor_pos.1) + 1;
                self.buffer.content[index..].fill(Cell::default());
            }
            ClearType::BeforeCursor => {
                let index = self.buffer.index_of(self.cursor_pos.0, self.cursor_pos.1);
                self.buffer.content[..index].fill(Cell::default());
            }
            ClearType::CurrentLine => {
                let line_start_index = self.buffer.index_of(0, self.cursor_pos.1);
                let line_end_index = self.buffer.index_of(self.width - 1, self.cursor_pos.1);
                self.buffer.content[line_start_index..=line_end_index].fill(Cell::default());
            }
            ClearType::UntilNewLine => {
                let index = self.buffer.index_of(self.cursor_pos.0, self.cursor_pos.1);
                let line_end_index = self.buffer.index_of(self.width - 1, self.cursor_pos.1);
                self.buffer.content[index..=line_end_index].fill(Cell::default());
            }
        }
        Ok(())
    }

    /// Inserts n line breaks at the current cursor position.
    ///
    /// After the insertion, the cursor x position will be incremented by 1 (unless it's already
    /// at the end of line). This is a common behaviour of terminals in raw mode.
    ///
    /// If the number of lines to append is fewer than the number of lines in the buffer after the
    /// cursor y position then the cursor is moved down by n rows.
    ///
    /// If the number of lines to append is greater than the number of lines in the buffer after
    /// the cursor y position then that number of empty lines (at most the buffer's height in this
    /// case but this limit is instead replaced with scrolling in most backend implementations) will
    /// be added after the current position and the cursor will be moved to the last row.
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

        Ok(())
    }

    fn size(&self) -> Result<Rect, io::Error> {
        Ok(Rect::new(0, 0, self.width, self.height))
    }

    fn window_size(&mut self) -> Result<WindowSize, io::Error> {
        // Some arbitrary window pixel size, probably doesn't need much testing.
        let window_pixel_size: Size = Size {
            width: 600,
            height: 480,
        };
        Ok(WindowSize {
            columns_rows: (self.width, self.height).into(),
            pixels: window_pixel_size,
        })
    }

    fn flush(&mut self) -> Result<(), io::Error> {
        Ok(())
    }
}
