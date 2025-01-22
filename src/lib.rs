use bevy::prelude::*;
use game::GameState;

mod assets;
mod camera;
mod enemies;
mod game;
mod game_over_menu;
mod hud;
mod input;
mod main_menu;
mod pause_menu;
mod player;
mod shoot;
mod ui_style;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>();
        app.add_plugins((
            camera::plugin,
            input::plugin,
            assets::plugin,
            game::plugin,
            main_menu::plugin,
            pause_menu::plugin,
            game_over_menu::plugin,
            player::plugin,
            hud::plugin,
            shoot::plugin,
            enemies::plugin,
        ));

        // // Enable dev tools for dev builds.
        // #[cfg(feature = "dev")]
        // app.add_plugins((dev_tools::plugin, debug::plugin));
    }
}
