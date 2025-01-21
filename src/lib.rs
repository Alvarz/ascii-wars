use bevy::prelude::*;

mod assets;
mod camera;
mod enemies;
mod game;
mod input;
mod main_menu;
mod player;
mod shoot;
mod ui_style;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    MainMenu,
    NewGame,
    Playing,
    PauseMenu,
}

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
            player::plugin,
            shoot::plugin,
            enemies::plugin,
        ));

        // // Enable dev tools for dev builds.
        // #[cfg(feature = "dev")]
        // app.add_plugins((dev_tools::plugin, debug::plugin));
    }
}
