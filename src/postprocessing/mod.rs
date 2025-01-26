use bevy::prelude::*;

pub mod example;
mod scanlines;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((example::PostProcessPlugin));
}
