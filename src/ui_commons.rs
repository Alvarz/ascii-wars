use bevy::prelude::*;

pub fn spawn_container(commands: &mut Commands) -> Entity {
    let container = Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        justify_content: JustifyContent::Center,
        ..default()
    };

    let e = commands.spawn(container).id();

    e
}
