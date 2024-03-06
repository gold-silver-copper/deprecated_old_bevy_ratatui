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
    pub true_color: BevyColor,
}

#[derive(Component, Debug, Clone, PartialEq)]
pub struct Cursor {
    pub pos: (u16, u16),
}

#[derive(Component, Debug, Clone, PartialEq)]
pub struct RapidBlink {
    pub in_blink: bool,
    pub true_color: BevyColor,
}

#[derive(Component, Debug, Clone, PartialEq)]
pub struct VirtualCell {
    pub symbol: String,
    pub fg: BevyColor,
    pub bg: BevyColor,
    pub underline_color: BevyColor,

    pub skip: bool,

    pub bold: bool,
    pub dim: bool,
    pub italic: bool,
    pub underlined: bool,
    pub slow_blink: bool,
    pub rapid_blink: bool,
    pub reversed: bool,
    pub hidden: bool,
    pub crossed_out: bool,

    pub row: u16,
    pub column: u16,
}

impl VirtualCell {
    pub fn new(x: u16, y: u16) -> Self {
        VirtualCell {
            symbol: "╬".to_string(),
            fg: bevy::prelude::Color::WHITE,
            bg: bevy::prelude::Color::BLACK,
            underline_color: bevy::prelude::Color::WHITE,
            skip: false,
            italic: false,
            underlined: false,
            bold: false,

            crossed_out: false,

            row: y,
            column: x,

            dim: false,
            reversed: false,

            slow_blink: false,
            rapid_blink: false,

            hidden: false,
        }
    }
}

pub trait FromRatCell {
    fn to_virtual(x: u16, y: u16, given_cell: &Cell) -> VirtualCell;
}

impl FromRatCell for VirtualCell {
    fn to_virtual(x: u16, y: u16, given_cell: &Cell) -> VirtualCell {
        VirtualCell {
            symbol: given_cell.symbol().into(),
            fg: BevyColor::from_rat_color(given_cell.fg, true),
            bg: BevyColor::from_rat_color(given_cell.bg, false),
            //    #[cfg(not(feature = "underline-color"))]
            underline_color: BevyColor::from_rat_color(given_cell.fg, true),
            //    #[cfg(feature = "underline-color")]
            //   underline_color: BevyColor::from_rat_color(given_cell.underline_color, true),
            bold: given_cell.modifier.intersects(Modifier::BOLD),
            dim: given_cell.modifier.intersects(Modifier::DIM),
            italic: given_cell.modifier.intersects(Modifier::ITALIC),
            underlined: given_cell.modifier.intersects(Modifier::UNDERLINED),
            slow_blink: given_cell.modifier.intersects(Modifier::SLOW_BLINK),
            rapid_blink: given_cell.modifier.intersects(Modifier::RAPID_BLINK),
            reversed: given_cell.modifier.intersects(Modifier::REVERSED),
            hidden: given_cell.modifier.intersects(Modifier::HIDDEN),
            crossed_out: given_cell.modifier.intersects(Modifier::CROSSED_OUT), /* FIX this SHOULD NOT BE ALL FALSE */

            skip: given_cell.skip,
            row: y,
            column: x,
        }
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
