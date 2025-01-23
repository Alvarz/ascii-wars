use bevy::prelude::*;

use crate::ui_style::{BOX_BG_COLOR, BOX_BORDER_COLOR, MAIN_TEXT_COLOR};

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

pub fn spawn_box(commands: &mut Commands, parent: Entity, width: Val, height: Val) -> Entity {
    let child = commands
        .spawn((
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                margin: UiRect::all(Val::Percent(5.)),
                width,
                height,
                border: UiRect::all(Val::Px(2.)),
                align_items: AlignItems::Center, // align horizontal
                // justify_content: JustifyContent::Center, // align vertical
                ..default()
            },
            BackgroundColor(BOX_BG_COLOR),
            BorderColor(BOX_BORDER_COLOR),
        ))
        .id();

    commands.entity(parent).add_children(&[child]);

    child
}

pub fn spawn_text(commands: &mut Commands, parent: Entity, text: &str, font_size: f32) -> Entity {
    //     let text = "ASCII Wars!";

    let child = commands
        .spawn((
            Text::new(text),
            TextFont {
                font_size,
                ..default()
            },
            TextColor(MAIN_TEXT_COLOR),
            TextLayout::new_with_justify(JustifyText::Center),
            Node {
                margin: UiRect::all(Val::Percent(10.)),
                position_type: PositionType::Relative,
                ..default()
            },
        ))
        .id();

    commands.entity(parent).add_children(&[child]);

    child
}
