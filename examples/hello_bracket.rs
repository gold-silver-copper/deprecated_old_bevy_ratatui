use bracket_lib::prelude::*;
use ratatui::{prelude::*, text::*, widgets::*};

use bracket_ratatui::BracketBackend;

//store the ratatui terminal inside bracket-lib state
struct State {
    my_terminal: Terminal<BracketBackend>,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        self.my_terminal.draw(|frame| {
            let area = frame.size();
            frame.render_widget(
                Paragraph::new("Hello Ratatui! (press 'q' to quit)")
                    .white()
                    .on_blue(),
                area,
            );
        });

        //ctx handles input but we draw to self.my_terminal

        match ctx.key {
            None => {}
            Some(key) => {
                if key == VirtualKeyCode::Q {
                    ctx.quit();
                    let x = &mut self.my_terminal.backend_mut().bracket_term;
                    x.quit();
                }
            }
        }

        //overwrite context , i do this due to lifetime reasons

        *ctx = self.my_terminal.backend().bracket_term.clone();
    }
}

fn main() -> BError {
    //create bracket terminal
    let context = BTermBuilder::simple80x50()
        .with_title("Hello Bracket Ratatui")
        .build()?;
    //create bracket state using bracket backend for ratatui
    let gs: State = State {
        my_terminal: Terminal::new(BracketBackend::new(context))?,
    };

    //create bracket loop , every loop the state terminal is drawn to then overrides loop context
    main_loop(gs.my_terminal.backend().bracket_term.clone(), gs)
}
