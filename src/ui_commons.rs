use bevy::prelude::*;

use crate::ui_style::{
    BOX_BG_COLOR, BOX_BORDER_COLOR, COLOR_TEXT_BUTTON, MAIN_TEXT_COLOR, NORMAL_BUTTON,
};

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

pub fn spawn_button<T: Bundle>(
    commands: &mut Commands,
    parent: Entity,
    text: String,
    btn_type: T,
    font_size: f32,
) -> Entity {
    let button = commands
        .spawn((
            Node {
                margin: UiRect::all(Val::Percent(1.)),
                position_type: PositionType::Relative,
                width: Val::Percent(60.),
                height: Val::Percent(20.),

                border: UiRect::all(Val::Px(2.0)),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            BorderColor(BOX_BORDER_COLOR),
            BackgroundColor(NORMAL_BUTTON),
            Button,
            btn_type,
        ))
        .id();

    let button_text = commands
        .spawn((
            TextColor(COLOR_TEXT_BUTTON),
            TextFont {
                font_size,
                ..default()
            },
            Text::new(text),
        ))
        .id();
    commands.entity(button).add_children(&[button_text]);
    commands.entity(parent).add_children(&[button]);

    button
}
