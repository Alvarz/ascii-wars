use bevy::prelude::*;

use crate::ui_commons::{spawn_box, spawn_container};
use crate::ui_style::{
    BOX_BG_COLOR, BOX_BORDER_COLOR, COLOR_TEXT_BUTTON, HOVERED_BUTTON, HOVER_TEXT_COLOR,
    MAIN_TEXT_COLOR, NORMAL_BUTTON, PRESSED_BUTTON,
};
use crate::GameState;

#[derive(Resource, Clone)]
pub struct PauseMenu {
    entity: Entity,
}

#[derive(Component, Clone)]
pub struct PlayAgainButton;

#[derive(Component, Clone)]
pub struct ExitButton;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::GameOver), game_over_menu);
    app.add_systems(
        Update,
        play_again_button_system.run_if(in_state(GameState::GameOver)),
    );

    app.add_systems(
        Update,
        exit_button_system.run_if(in_state(GameState::GameOver)),
    );
    app.add_systems(OnExit(GameState::GameOver), clear_game_over_menu);
}

fn game_over_menu(mut commands: Commands) {
    let container = spawn_container(&mut commands);
    commands.insert_resource(PauseMenu { entity: container });

    let menu_box = spawn_box(
        &mut commands,
        container,
        Val::Percent(40.),
        Val::Percent(60.),
    );

    spawn_text(&mut commands, menu_box);
    spawn_button(
        &mut commands,
        menu_box,
        "Play Again".to_string(),
        PlayAgainButton,
    );
    spawn_button(&mut commands, menu_box, "Exit".to_string(), ExitButton);
}

fn clear_game_over_menu(mut commands: Commands, menu: Res<PauseMenu>) {
    commands.entity(menu.entity).despawn_recursive();
    commands.remove_resource::<PauseMenu>();
}

// fn spawn_box(commands: &mut Commands, parent: Entity) -> Entity {
//     let child = commands
//         .spawn((
//             Node {
//                 display: Display::Flex,
//                 flex_direction: FlexDirection::Column,
//                 margin: UiRect::all(Val::Percent(5.)),
//                 width: Val::Percent(40.),
//                 height: Val::Percent(60.),
//                 border: UiRect::all(Val::Px(2.)),
//                 align_items: AlignItems::Center, // align horizontal
//                 // justify_content: JustifyContent::Center, // align vertical
//                 ..default()
//             },
//             BackgroundColor(BOX_BG_COLOR),
//             BorderColor(BOX_BORDER_COLOR),
//         ))
//         .id();

//     commands.entity(parent).add_children(&[child]);

//     child
// }

fn spawn_text(commands: &mut Commands, parent: Entity) {
    let text = "Game Over";

    let child = commands
        .spawn((
            Text::new(text),
            TextFont {
                font_size: 20.0,
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
}

fn spawn_button<T>(commands: &mut Commands, parent: Entity, text: String, btn_type: T) -> Entity
where
    T: Bundle,
{
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
                font_size: 18.0,
                ..default()
            },
            Text::new(text),
        ))
        .id();
    commands.entity(button).add_children(&[button_text]);
    commands.entity(parent).add_children(&[button]);

    button
}

fn play_again_button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
            &PlayAgainButton,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_color_query: Query<&mut TextColor>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color, mut border_color, children, _) in &mut interaction_query {
        let mut text_color = text_color_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = BOX_BORDER_COLOR.into();
                *text_color = HOVER_TEXT_COLOR.into();
                next_state.set(GameState::NewGame)
            }

            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = BOX_BORDER_COLOR.into();
                *text_color = HOVER_TEXT_COLOR.into()
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = BOX_BORDER_COLOR.into();
                *text_color = MAIN_TEXT_COLOR.into()
                // border_color.0 = NORMAL_BUTTON.into();
            }
        }
    }
}

fn exit_button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
            &ExitButton,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_color_query: Query<&mut TextColor>,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, mut color, mut border_color, children, _) in &mut interaction_query {
        let mut text_color = text_color_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = BOX_BORDER_COLOR.into();
                *text_color = HOVER_TEXT_COLOR.into();
                exit.send(AppExit::Success);
            }

            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = BOX_BORDER_COLOR.into();
                *text_color = HOVER_TEXT_COLOR.into()
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = BOX_BORDER_COLOR.into();
                *text_color = MAIN_TEXT_COLOR.into()
                // border_color.0 = NORMAL_BUTTON.into();
            }
        }
    }
}
