use bevy::prelude::*;
use game::GameState;

mod assets;
mod camera;
mod enemies;
mod game;
mod input;
mod player;
mod postprocessing;
mod shoot;
mod ui;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>();
        app.add_plugins((
            camera::plugin,
            input::plugin,
            assets::plugin,
            game::plugin,
            player::plugin,
            shoot::plugin,
            enemies::plugin,
            ui::plugin,
            postprocessing::plugin,
        ));

        // // Enable dev tools for dev builds.
        // #[cfg(feature = "dev")]
        // app.add_plugins((dev_tools::plugin, debug::plugin));
    }
}
