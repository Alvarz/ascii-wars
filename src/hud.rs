use bevy::prelude::*;

use crate::{
    game::Pool,
    player::Player,
    ui_style::{BOX_BG_COLOR, BOX_BORDER_COLOR, HEALTH_BAR_COLOR, MAIN_TEXT_COLOR},
    GameState,
};

#[derive(Resource, Clone)]
pub struct Hud {
    entity: Entity,
}

#[derive(Component, Clone)]
pub struct HealthBar;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Playing), hud);
    app.add_systems(Update, check_health.run_if(in_state(GameState::Playing)));

    //     app.add_systems(OnEnter(GameState::MainMenu), clear_hud);
}

fn hud(mut commands: Commands) {
    let container = spawn_container(&mut commands);
    let _ = spawn_health_bar(&mut commands, container);
    spawn_text(&mut commands, container);
}

fn clear_hud(mut commands: Commands, menu: Res<Hud>) {
    commands.entity(menu.entity).despawn_recursive();
    commands.remove_resource::<Hud>();
}

fn spawn_container(commands: &mut Commands) -> Entity {
    let container = Node {
        display: Display::Flex,
        flex_direction: FlexDirection::Row,
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        justify_content: JustifyContent::Center, // align vertical
        ..default()
    };

    let e = commands.spawn(container).id();

    commands.insert_resource(Hud { entity: e });

    e
}

fn spawn_health_bar(commands: &mut Commands, parent: Entity) -> Entity {
    let health_bar = commands
        .spawn((
            HealthBar,
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                margin: UiRect::all(Val::Percent(1.)),
                width: Val::Percent(25.),
                height: Val::Percent(5.),
                border: UiRect::all(Val::Px(2.)),
                align_items: AlignItems::Center, // align horizontal
                ..default()
            },
            BackgroundColor(BOX_BG_COLOR),
            BorderColor(BOX_BORDER_COLOR),
        ))
        .id();

    let health_bar_content = commands
        .spawn((
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                width: Val::Percent(75.),
                height: Val::Percent(100.),
                align_items: AlignItems::Center, // align horizontal
                ..default()
            },
            BackgroundColor(HEALTH_BAR_COLOR),
            BorderColor(HEALTH_BAR_COLOR),
        ))
        .id();

    commands
        .entity(health_bar)
        .add_children(&[health_bar_content]);

    commands.entity(parent).add_children(&[health_bar]);

    health_bar
}

fn spawn_text(commands: &mut Commands, parent: Entity) {
    let text = "level: 1";

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
                margin: UiRect {
                    left: Val::Percent(20.),
                    right: Val::Percent(48.),
                    top: Val::Percent(1.),
                    bottom: Val::Percent(0.),
                },
                position_type: PositionType::Relative,
                ..default()
            },
        ))
        .id();

    commands.entity(parent).add_children(&[child]);
}

fn check_health(
    pool_query: Single<&Pool, With<Player>>,
    mut health_bar_query: Single<(&Node, &mut Children), With<HealthBar>>,
    mut health_bar_content_query: Query<&mut Node, Without<HealthBar>>,
) {
    //     health_bar

    let children = &mut health_bar_query.1;

    let mut health_bar_content = health_bar_content_query.get_mut(children[0]).unwrap();

    let current_health = (pool_query.health / pool_query.max_health) * 100.;
    health_bar_content.width = Val::Percent(current_health.clamp(0., 100.));
}
