

use bevy::{
    prelude::{Color as BevyColor, *},
    time::common_conditions::on_timer,
    utils::{Duration, HashMap},
    window::{PrimaryWindow, WindowResized, WindowResolution},
};

use crate::components::{CellComponent, RapidBlink, SlowBlink, TerminalComponent};
use crate::FontStyle;

///Provides Bevy Plugin which creates terminal like window supporting Ratatui
pub struct RatatuiPlugin;

impl Plugin for RatatuiPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<TermState>();
        app.init_state::<TermSizing>();

        app.add_systems(
            First,
            slow_blink_cells.run_if(on_timer(Duration::from_secs_f32(0.6))),
        );
        app.add_systems(
            First,
            rapid_blink_cells.run_if(on_timer(Duration::from_millis(200))),
        );

        app.add_systems(
            First,
            (font_setup
                .after(query_term_for_init)
                .run_if(in_state(TermState::TermNeedsFont))),
        );
        app.add_systems(
            First,
            (clear_virtual_cells
                .after(font_setup)
                .run_if(in_state(TermState::TermNeedsClearing))),
        );
        app.add_systems(
            First,
            (init_virtual_cells
                .after(clear_virtual_cells)
                .run_if(in_state(TermState::TermNeedsIniting))),
        );

        app.add_systems(
            First,
            (do_first_resize.run_if(in_state(TermState::AllTermsInited)))
                .run_if(in_state(TermSizing::TermNeedsFirstResize)),
        );

        app.add_systems(First, (query_term_for_init));

        app.add_systems(
            Last,
            (handle_primary_window_resize).run_if(on_event::<WindowResized>()),
        );
        app.add_systems(
            PostUpdate,
            (update_ents_from_vcupdate).run_if(in_state(TermState::AllTermsInited)),
        );
        app.add_systems(
            PostUpdate,
            (update_ents_from_vcupdate).run_if(in_state(TermState::TermNeedsIniting)),
        );
        app.add_systems(
            Update,
            (debug_entities).run_if(in_state(TermState::AllTermsInited)),
        );

        app.add_systems(
            First,
            (update_ents_from_comp)
                .after(update_ents_from_vcupdate)
                .run_if(in_state(TermState::AllTermsInited)),
        );

        app.add_systems(
            Last,
            (update_cursor)
                .after(handle_primary_window_resize)
                .run_if(in_state(TermState::AllTermsInited))
                .run_if(in_state(TermSizing::TermGood)),
        );
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
enum TermState {
    #[default]
    NoTermsInited,
    TermNeedsFont,
    TermNeedsClearing,

    TermNeedsIniting,
    AllTermsInited,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
enum TermSizing {
    #[default]
    NoTermsInited,

    TermNeedsFirstResize,
    TermGood,
}

fn do_first_resize(
    mut commands: Commands,
    mut windows: Query<&mut Window>,
    mut terminal_query: Query<(&Node, Entity, &mut TerminalComponent)>,
    mut resize_state: ResMut<NextState<TermSizing>>,
) {
    let mut window = windows.single_mut();

    let (nodik, e, mut termy) = terminal_query
        .get_single_mut()
        .expect("More than one terminal with a bevybackend");

    let ns = termy.get_text_style(BevyColor::DARK_GRAY, FontStyle::Normal);

    let rat_term = &mut termy.ratatui_terminal;
    let termy_backend = rat_term.backend_mut();
    let rows = termy_backend.height;
    let columns = termy_backend.width;

    let node_size = nodik.size();

    window
        .resolution
        .set(node_size.x * columns as f32, node_size.y * rows as f32);
    //spawn the cursor

    let cursor_cell = commands
        .spawn((TextBundle::from_section(" ", ns).with_style(Style {
            top: Val::Px(termy_backend.cursor_pos.1 as f32 * node_size.y),
            left: Val::Px(termy_backend.cursor_pos.0 as f32 * node_size.x),

            ..default()
        }),))
        .id();

    termy_backend.cursor_ref = cursor_cell;

    resize_state.set(TermSizing::TermGood);
}

fn slow_blink_cells(
    mut slow_blink_query: Query<((&mut Text, &BackgroundColor, &mut SlowBlink))>,
) {
    for (mut text, bgc, mut sb) in slow_blink_query.iter_mut() {
        let mut section = text.sections.pop().unwrap();

       

        if sb.in_blink {
            sb.in_blink = false;
            section.style.color = bgc.0;
        } else {
            sb.in_blink = true;
            section.style.color = sb.true_color.clone();
        }

        text.sections.push(section);
    }
}

fn rapid_blink_cells(
    mut rapid_blink_query: Query<((&mut Text, &BackgroundColor, &mut RapidBlink))>,
) {
    for (mut text, bgc, mut sb) in rapid_blink_query.iter_mut() {
        let mut section = text.sections.pop().unwrap();

        

        if sb.in_blink {
            sb.in_blink = false;
            section.style.color = bgc.0;
        } else {
            sb.in_blink = true;
            section.style.color = sb.true_color.clone();
        }

        text.sections.push(section);
    }
}

fn query_term_for_init(
    mut terminal_query: Query<(&mut TerminalComponent)>,
    mut app_state: ResMut<NextState<TermState>>,
    mut resize_state: ResMut<NextState<TermSizing>>,
) {
    let mut termy = &mut terminal_query
        .get_single_mut()
        .expect("More than one terminal with a bevybackend")
        .ratatui_terminal;
    let termy_backend = termy.backend_mut();

    if (termy_backend.bevy_initialized == false) {
        app_state.set(TermState::TermNeedsFont);
        resize_state.set(TermSizing::TermNeedsFirstResize);
        termy_backend.bevy_initialized = true;
    }
}

fn clear_virtual_cells(
    mut commands: Commands,
    mut terminal_query: Query<(Entity, &mut TerminalComponent)>,
    mut app_state: ResMut<NextState<TermState>>,
) {
    let (e, mut termy) = terminal_query
        .get_single_mut()
        .expect("More than one terminal with a bevybackend");

    let ns = termy.get_text_style(BevyColor::DARK_GRAY, FontStyle::Normal);
    let rat_term = &mut termy.ratatui_terminal;
    let termy_backend = rat_term.backend_mut();

    for (_, entity) in termy_backend.entity_map.iter() {
        commands.entity(*entity).despawn();
    }
    termy_backend.entity_map = HashMap::new();

    // spawn a default node for the terminal to reference
    commands.entity(e).insert(
        TextBundle::from_section("T", ns) // Set the justification of the Text
            .with_background_color(BevyColor::DARK_GRAY)
            .with_text_justify(JustifyText::Center)
            .with_style(Style {
                display: Display::Grid,
                position_type: PositionType::Absolute,
                align_items: AlignItems::Stretch,
                margin: UiRect::ZERO,
                padding: UiRect::ZERO,
                border: UiRect::ZERO,
                grid_auto_flow: GridAutoFlow::Column,
                top: Val::Px(-30.0),
                left: Val::Px(-30.0),

                ..default()
            }),
    );

    app_state.set(TermState::TermNeedsIniting);
}

fn update_cursor(
    terminal_query: Query<(&Node, Entity, &TerminalComponent)>,

    mut commands: Commands,
) {
    let (nodik, e, termy) = terminal_query
        .get_single()
        .expect("More than one terminal with a bevybackend");
    let ns = termy.get_text_style(BevyColor::GREEN, FontStyle::Normal);
    let rat_term = &termy.ratatui_terminal;
    let termy_backend = rat_term.backend();
    let node_size = nodik.size();

    commands
        .entity(termy_backend.cursor_ref)
        .insert((TextBundle::from_section(" ", ns).with_style(Style {
            top: Val::Px(termy_backend.cursor_pos.1 as f32 * node_size.y),
            left: Val::Px(termy_backend.cursor_pos.0 as f32 * node_size.x),

            ..default()
        }),));

    if termy_backend.cursor {
        commands
            .entity(termy_backend.cursor_ref)
            .insert(Visibility::Visible);
    } else {
        commands
            .entity(termy_backend.cursor_ref)
            .insert(Visibility::Hidden);
    }
}

fn init_virtual_cells(
    mut commands: Commands,
    mut terminal_query: Query<(&Node, Entity, &mut TerminalComponent)>,
    mut app_state: ResMut<NextState<TermState>>,
) {
    let (nodik, e, mut termy) = terminal_query
        .get_single_mut()
        .expect("More than one terminal with a bevybackend");
    let ns = termy.get_text_style(BevyColor::DARK_GRAY, FontStyle::Normal);
    let rat_term = &mut termy.ratatui_terminal;

    let termy_backend = rat_term.backend_mut();
    let rows = termy_backend.height;
    let columns = termy_backend.width;
    termy_backend.entity_map = HashMap::new();

    let node_size = nodik.size();

    for y in 0..rows {
        for x in 0..columns {
            let ratcell = termy_backend.buffer.get(x, y);
            let vcell = commands
                .spawn((
                    CellComponent::from_cell(ratcell),
                    TextBundle::from_section(ratcell.symbol(), ns.clone()).with_style(Style {
                        top: Val::Px(y as f32 * node_size.y),
                        left: Val::Px(x as f32 * node_size.x),

                        ..default()
                    }),
                ))
                .id();

            termy_backend.entity_map.insert((x, y), vcell);
        }
    }

    app_state.set(TermState::AllTermsInited);
}

fn update_ents_from_vcupdate(
    mut commands: Commands,
    mut terminal_query: Query<(&mut TerminalComponent)>,
) {
    let mut termy = &mut terminal_query
        .get_single_mut()
        .expect("More than one terminal with a bevybackend")
        .ratatui_terminal;
    let termy_backend = termy.backend_mut();
    let boop = termy_backend.entity_map.clone();

    while let Some((x, y, vc)) = termy_backend.vcupdate.pop() {
        let xy = (x.clone(), y.clone());
        match boop.get(&xy) {
            Some(wow) => {
                commands
                    .entity(*wow)
                    .insert(CellComponent::from_cell(&vc));
                ()
            }
            None => (),
        };
    }
}

fn handle_primary_window_resize(
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    mut terminal_query: Query<(&mut TerminalComponent, &Node)>,
    mut resize_event: EventReader<WindowResized>,
    mut app_state: ResMut<NextState<TermState>>,
) {
    if let Ok((mut termy, nodik)) = terminal_query.get_single_mut() {
        for wr in resize_event.read() {
            let termy_backend = termy.ratatui_terminal.backend_mut();

            if !termy_backend.manual_window_sizing {
                let node_size = nodik.size();

                let w_wid = node_size.x;
                let w_hei = node_size.y;

                let new_wid = (wr.width / w_wid) as u16;
                let new_hei = (wr.height / w_hei) as u16;

                termy_backend.resize(new_wid as u16, new_hei as u16);
                app_state.set(TermState::TermNeedsClearing);

                for mut window in windows.iter_mut() {
                    window.resolution =
                        WindowResolution::new(new_wid as f32 * w_wid, new_hei as f32 * w_hei);

                    // Query returns one window typically.
                }
            }
        }
    }
}

fn debug_entities(query_cells: Query<(Entity, &Node)>) {
    for (entity_id, cs) in query_cells.iter() {

        //   println!("the calculated size is {:?}",cs.size());
    }
}

fn update_ents_from_comp(
    //this should run after update from vcbuffer
    query_cells: Query<
        (
            Entity,
            &CellComponent,
            &Style,
            Option<&SlowBlink>,
            Option<&RapidBlink>,
        ),
        (Changed<CellComponent>),
    >,
    mut commands: Commands,
    terminal_query: Query<((&TerminalComponent))>,
) {
    let termy = terminal_query
        .get_single()
        .expect("More than one terminal with a bevybackend");
    let termy_backend = termy.ratatui_terminal.backend();
    let fontsize = termy_backend.term_font_size as f32;

    for (entity_id, cellii, stylik, sbo, rbo) in query_cells.iter() {
        if !cellii.skip() {
            let (proper_fg, proper_bg) = cellii.proper_fg_bg();
            let mut ns = TextStyle::default();

            if (cellii.bold() && cellii.italic()) {
                ns = termy.get_text_style(proper_fg.clone(), FontStyle::ItalicBold);
            } else if (cellii.bold()) {
                ns = termy.get_text_style(proper_fg.clone(), FontStyle::Bold);
            } else if (cellii.italic()) {
                ns = termy.get_text_style(proper_fg.clone(), FontStyle::Italic);
            } else {
                ns = termy.get_text_style(proper_fg.clone(), FontStyle::Normal);
            }

            if let Some(x) = sbo {
                if !cellii.slow_blink() {
                    commands.entity(entity_id).remove::<SlowBlink>();
                }
            } else if cellii.slow_blink() {
                commands.entity(entity_id).insert(SlowBlink {
                    in_blink: false,
                    true_color: proper_fg.clone(),
                });
            } else {
                commands.entity(entity_id).remove::<SlowBlink>();
            }

            if let Some(x) = rbo {
                if !cellii.rapid_blink() {
                    commands.entity(entity_id).remove::<RapidBlink>();
                }
            } else if cellii.rapid_blink() {
                commands.entity(entity_id).insert(RapidBlink {
                    in_blink: false,
                    true_color: proper_fg.clone(),
                });
            } else {
                commands.entity(entity_id).remove::<RapidBlink>();
            }

            commands.entity(entity_id).insert(
                TextBundle::from_section(cellii.proper_symbol(), ns)
                    .with_background_color(proper_bg)
                    .with_text_justify(JustifyText::Center)
                    .with_style(stylik.clone()),
            );
        }
    }
}

fn font_setup(
    asset_server: Res<AssetServer>,
    mut terminal_query: Query<((Entity, &mut TerminalComponent))>,
    mut app_state: ResMut<NextState<TermState>>,
) {
    let (e, mut termy) = terminal_query
        .get_single_mut()
        .expect("More than one terminal with a bevybackend");
    let mut termy_backend = termy.ratatui_terminal.backend_mut();

    if let Some(x) = &termy_backend.normal_font_path {
        termy_backend.normal_handle = asset_server.load(x);
    }
    if let Some(x) = &termy_backend.italic_font_path {
        termy_backend.italic_handle = asset_server.load(x);
    }
    if let Some(x) = &termy_backend.bold_font_path {
        termy_backend.bold_handle = asset_server.load(x);
    }
    if let Some(x) = &termy_backend.italicbold_font_path {
        termy_backend.italicbold_handle = asset_server.load(x);
    }

    app_state.set(TermState::TermNeedsClearing);
}
