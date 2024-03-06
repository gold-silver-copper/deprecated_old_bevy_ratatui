use bevy::prelude::{Color as BevyColor, Component};

use ratatui::{
    buffer::Cell,
    style::{Color as RatColor, Modifier},
    terminal::Terminal as RatTerminal,
};

use crate::BevyBackend;

#[derive(Component, Debug, Clone)]
pub struct TerminalComponent {
    pub ratatui_terminal: RatTerminal<BevyBackend>,
}

#[derive(Component, Debug, Clone, PartialEq)]
pub struct SlowBlink {
    pub in_blink: bool,
    pub true_color: RatColor,
}

#[derive(Component, Debug, Clone, PartialEq)]
pub struct Cursor {
    pub pos: (u16, u16),
}

#[derive(Component, Debug, Clone, PartialEq)]
pub struct RapidBlink {
    pub in_blink: bool,
    pub true_color: RatColor,
}

#[derive(Component, Debug, Clone)]
pub struct CellComponent {
    pub cell: Cell,
    pub row: u16,
    pub column: u16,
}

impl CellComponent {
    pub fn new(x: u16, y: u16) -> Self {
        CellComponent {
            cell: Cell::default(),
            row: y,
            column: x,
        }
    }

    pub fn from_cell(x: u16, y: u16, cell: &Cell) -> Self {
        CellComponent {
            cell: cell.clone(),
            row: y,
            column: x,
        }
    }

    pub fn set_fg_to_bg() {}

    pub fn fg(&self) -> BevyColor {
        BevyColor::from_rat_color(self.cell.fg, true)
    }

    pub fn bg(&self) -> BevyColor {
        BevyColor::from_rat_color(self.cell.bg, false)
    }

    pub fn bold(&self) -> bool {
        self.cell.modifier.intersects(Modifier::BOLD)
    }
    pub fn dim(&self) -> bool {
        self.cell.modifier.intersects(Modifier::DIM)
    }
    pub fn italic(&self) -> bool {
        self.cell.modifier.intersects(Modifier::ITALIC)
    }
    pub fn underlined(&self) -> bool {
        self.cell.modifier.intersects(Modifier::UNDERLINED)
    }
    pub fn slow_blink(&self) -> bool {
        self.cell.modifier.intersects(Modifier::SLOW_BLINK)
    }

    pub fn rapid_blink(&self) -> bool {
        self.cell.modifier.intersects(Modifier::RAPID_BLINK)
    }
    pub fn reversed(&self) -> bool {
        self.cell.modifier.intersects(Modifier::REVERSED)
    }
    pub fn hidden(&self) -> bool {
        self.cell.modifier.intersects(Modifier::HIDDEN)
    }
    pub fn crossed_out(&self) -> bool {
        self.cell.modifier.intersects(Modifier::CROSSED_OUT)
    }
    pub fn skip(&self) -> bool {
        self.cell.skip
    }

    pub fn x(&self) -> u16 {
        self.column
    }
    pub fn y(&self) -> u16 {
        self.row
    }
}

impl FromRatColor<RatColor> for BevyColor {
    fn from_rat_color(color: RatColor, fg: bool) -> Self {
        match color {
            RatColor::Reset => {
                if fg {
                    BevyColor::WHITE
                } else {
                    BevyColor::DARK_GRAY
                }
            }
            RatColor::Black => BevyColor::BLACK,
            RatColor::Red => BevyColor::MAROON,
            RatColor::Green => BevyColor::DARK_GREEN,
            RatColor::Yellow => BevyColor::GOLD,
            RatColor::Blue => BevyColor::MIDNIGHT_BLUE,
            RatColor::Magenta => BevyColor::FUCHSIA,
            RatColor::Cyan => BevyColor::CYAN,
            RatColor::Gray => BevyColor::GRAY,
            RatColor::DarkGray => BevyColor::DARK_GRAY,
            RatColor::LightRed => BevyColor::RED,
            RatColor::LightGreen => BevyColor::GREEN,
            RatColor::LightBlue => BevyColor::BLUE,
            RatColor::LightYellow => BevyColor::BISQUE,
            RatColor::LightMagenta => BevyColor::PINK,
            RatColor::LightCyan => BevyColor::AQUAMARINE,
            RatColor::White => BevyColor::WHITE,
            RatColor::Indexed(i) => BevyColor::from_ansi(i),
            RatColor::Rgb(r, g, b) => BevyColor::rgb_u8(r, g, b),
        }
    }
}

trait FromAnsi<u8> {
    fn from_ansi(beep: u8) -> BevyColor;
}

impl FromAnsi<u8> for BevyColor {
    fn from_ansi(beep: u8) -> BevyColor {
        BevyColor::rgb_u8(beep, beep, beep)
    }
}

trait FromRatColor<RatColor> {
    fn from_rat_color(color: RatColor, fg: bool) -> BevyColor;
}