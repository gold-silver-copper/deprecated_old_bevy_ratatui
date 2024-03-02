use bracket_lib::prelude::*;
use std::{error::Error, time::Duration};

use once_cell::sync::Lazy;
use ratatui::prelude::*;

use bracket_ratatui::BracketBackend;

use crate::{app::App as RatApp, ui};

static mut RATAPP: Lazy<RatApp> = Lazy::new(|| RatApp::new("BEVY Demo", true));

unsafe fn get_ratapp() -> &'static mut RatApp<'static> {
    return &mut RATAPP;
}
bracket_lib::prelude::embedded_resource!(TILE_FONT3, "resources/unicode_16x16.png");

struct State {
    my_terminal: Terminal<BracketBackend>,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        self.my_terminal
            .backend_mut()
            .bracket_term
            .set_translation_mode(0, CharacterTranslationMode::Unicode);

        let mut ra = unsafe { get_ratapp() };

        let fpsik = ctx.fps;

        ra.on_tick();

        self.my_terminal.draw(|f| ui::draw(f, &mut ra));

        let mut input = INPUT.lock();
        ctx.print(1, 3, &format!("FPS: {}", fpsik));
        for (i, key) in input.key_pressed_set().iter().enumerate() {
            match key {
                VirtualKeyCode::Q => {
                    self.my_terminal.backend_mut().bracket_term.quit();
                }
                VirtualKeyCode::H => {
                    ra.on_left();
                }
                VirtualKeyCode::J => {
                    ra.on_up();
                }
                VirtualKeyCode::K => {
                    ra.on_down();
                }
                VirtualKeyCode::L => {
                    ra.on_right();
                }
                VirtualKeyCode::L => {
                    ra.on_right();
                }
                VirtualKeyCode::T => {
                    ra.on_key("t".chars().next().unwrap());
                }
                VirtualKeyCode::C => {
                    ra.on_key("c".chars().next().unwrap());
                }

                _ => {}
            }
        }

        *ctx = self.my_terminal.backend().bracket_term.clone();
    }
}

pub fn run(ticky_rate: Duration, enhanced_graphics: bool) -> Result<(), Box<dyn Error>> {
    bracket_lib::prelude::link_resource!(TILE_FONT3, "resources/unicode_16x16.png");

    let context = BTermBuilder::new()
        .with_dimensions(80, 50)
        .with_tile_dimensions(16, 16)
        .with_title("Hello Minimal Bracket World")
        .with_font("unicode_16x16.png", 16, 16)
        .with_simple_console(80, 50, "unicode_16x16.png")
        .build()
        .unwrap();

    let ratapp = RatApp::new("H͟e͟l͟l͟o͟ ͟t͟e͟x͟t͟ ͟s͟t͟r͟i͟n͟g͟ ͟m͟y͟ ͟o͟l͟d͟ ͟f͟r͟i͟e͟n͟d͟.͟", enhanced_graphics);
    let mut ra = unsafe { get_ratapp() };
    *ra = ratapp;

    let gs: State = State {
        my_terminal: Terminal::new(BracketBackend::new(context))?,
    };
    main_loop(gs.my_terminal.backend().bracket_term.clone(), gs);

    Ok(())
}
