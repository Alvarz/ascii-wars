use bevy::prelude::*;

mod finished_level;
mod game_over_menu;
mod hud;
mod main_menu;
mod pause_menu;
mod ui_commons;
mod ui_style;
mod winner_page;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        main_menu::plugin,
        pause_menu::plugin,
        game_over_menu::plugin,
        winner_page::plugin,
        hud::plugin,
        finished_level::plugin,
    ));
}
