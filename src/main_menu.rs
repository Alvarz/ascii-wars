use bevy::color::palettes::css::{BLACK, BLUE, WHITE};
use bevy::prelude::*;

use crate::GameState;

#[derive(Resource, Clone)]
pub struct MainMenu {
    pub entity: Entity,
}

const NORMAL_BUTTON: Color = bevy::prelude::Color::Srgba(BLACK);
const PRESSED_BUTTON: Color = bevy::prelude::Color::Srgba(BLUE);
const HOVERED_BUTTON: Color = bevy::prelude::Color::Srgba(WHITE);

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::MainMenu), main_menu);
    app.add_systems(Update, button_system.run_if(in_state(GameState::MainMenu)));
    app.add_systems(OnExit(GameState::MainMenu), clear_main_menu);
}

fn main_menu(mut commands: Commands) {
    let container = spawn_container(&mut commands);
    let menu_box = spawn_box(&mut commands, container);
    spawn_text(&mut commands, menu_box);
    spawn_button(&mut commands, menu_box);
}

fn spawn_container(commands: &mut Commands) -> Entity {
    let container = Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        justify_content: JustifyContent::Center,
        ..default()
    };

    let e = commands.spawn(container).id();

    commands.insert_resource(MainMenu { entity: e });

    e
}

fn clear_main_menu(mut commands: Commands, menu: Res<MainMenu>) {
    commands.entity(menu.entity).despawn_recursive();
    commands.remove_resource::<MainMenu>();
}

fn spawn_text(commands: &mut Commands, parent: Entity) {
    let text = "Hello world!";

    let child = commands
        .spawn((
            Text::new(text),
            TextFont {
                font_size: 100.0,
                ..default()
            },
            TextColor(Color::WHITE),
            TextLayout::new_with_justify(JustifyText::Center),
            Node {
                position_type: PositionType::Relative,
                margin: UiRect {
                    left: Val::Px(0.),
                    right: Val::Px(0.),
                    top: Val::Px(20.),
                    bottom: Val::Px(0.),
                },
                ..default()
            },
        ))
        .id();

    commands.entity(parent).add_children(&[child]);
}

fn spawn_box(commands: &mut Commands, parent: Entity) -> Entity {
    let square = Node {
        margin: UiRect {
            left: Val::Px(0.),
            right: Val::Px(0.),
            top: Val::Px(20.),
            bottom: Val::Px(0.),
        },
        width: Val::Percent(60.),
        height: Val::Percent(60.),
        border: UiRect::all(Val::Px(2.)),
        ..default()
    };

    let square_color = Color::srgb(0.65, 0.65, 0.65);

    let child = commands.spawn((square, BackgroundColor(square_color))).id();

    commands.entity(parent).add_children(&[child]);

    child
}

fn spawn_button(commands: &mut Commands, parent: Entity) -> Entity {
    let button_node = Node {
        width: Val::Px(150.0),
        height: Val::Px(65.0),
        border: UiRect::all(Val::Px(5.0)),
        // horizontally center child text
        justify_content: JustifyContent::Center,
        // vertically center child text
        align_items: AlignItems::Center,
        ..default()
    };

    let button_text_node = Text::new("Button");
    let button_text_color = TextColor(Color::srgb(0.9, 0.9, 0.9));
    let button_text_font = TextFont {
        font_size: 40.0,
        ..default()
    };

    let button = commands
        .spawn((
            button_node,
            BorderColor(Color::BLACK),
            BackgroundColor(NORMAL_BUTTON),
            Button,
        ))
        .id();

    let button_text = commands
        .spawn((button_text_node, button_text_color, button_text_font))
        .id();
    commands.entity(button).add_children(&[button_text]);
    commands.entity(parent).add_children(&[button]);

    button
}

fn button_system(
    mut commands: Commands,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut next_state: ResMut<NextState<GameState>>,
    main_menu: Res<MainMenu>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                text.0 = "Press".to_string();
                *color = PRESSED_BUTTON.into();
                border_color.0 = BLUE.into();
                next_state.set(GameState::NewGame)
            }

            Interaction::Hovered => {
                text.0 = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
                border_color.0 = WHITE.into();
            }
            Interaction::None => {
                text.0 = "Button".to_string();
                *color = NORMAL_BUTTON.into();
                border_color.0 = BLACK.into();
            }
        }
    }
}
