use bevy::{prelude::*, window::PrimaryWindow};

use crate::{assets::CharsetAsset, enemies::spawn_boss, player::spawn_player};

#[derive(Component)]
pub struct GamePlayEntity;

#[derive(Component)]
pub struct Pool {
    pub health: f32,
    pub max_health: f32,
    pub damage: f32,
    pub god_mode: bool,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    MainMenu,
    NewGame,
    Playing,
    PauseMenu,
    GameOver,
    NextLevel,
    WinGame,
    FinishedLevel,
}

#[derive(Resource, Debug)]
pub struct GameStatus {
    pub level: i32,
    pub max_level: i32,
}

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(GameStatus {
        level: 0,
        max_level: 10,
    });

    app.add_systems(OnEnter(GameState::GameOver), clean_up_level);

    app.add_systems(
        OnEnter(GameState::NewGame),
        (new_game, prepare_level).chain(),
    );

    app.add_systems(
        OnEnter(GameState::NextLevel),
        (clean_up_level, prepare_level).chain(),
    );
}

fn new_game(mut game_status: ResMut<GameStatus>) {
    game_status.level = 0;
}

fn prepare_level(
    mut game_status: ResMut<GameStatus>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    chaset: Res<CharsetAsset>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    if game_status.level >= game_status.max_level {
        next_state.set(GameState::WinGame);
        return;
    }

    game_status.level += 1;

    spawn_player(&mut commands, &chaset, window);

    spawn_level_boss(&mut commands, &chaset, window, game_status.level);

    next_state.set(GameState::Playing);
}

fn spawn_level_boss(commands: &mut Commands, chaset: &CharsetAsset, window: &Window, level: i32) {
    spawn_boss(commands, chaset, window, level as usize);
}

fn clean_up_level(mut commands: Commands, mut entities: Query<Entity, With<GamePlayEntity>>) {
    for e in &mut entities {
        commands.entity(e).despawn();
    }
}
