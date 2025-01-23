use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    assets::CharsetAsset,
    game::{GamePlayEntity, GameState, Pool},
    shoot::spawn_bullet,
};

#[derive(Component)]
pub struct Boss;

#[derive(Component)]
enum Direction {
    Up,
    Down,
}

#[derive(Component)]
pub struct ShootPattern1;

#[derive(Component)]
pub struct ShootPattern2;

const BOSSES_GLYPH: [usize; 11] = [
    'a' as usize,
    'A' as usize,
    'B' as usize,
    'C' as usize,
    'D' as usize,
    'E' as usize,
    'F' as usize,
    'G' as usize,
    'H' as usize,
    'I' as usize,
    'J' as usize,
];

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        boss_shoot_pattern_1.run_if(in_state(GameState::Playing)),
    );

    app.add_systems(
        Update,
        boss_shoot_pattern_2.run_if(in_state(GameState::Playing)),
    );
    app.add_systems(Update, movement.run_if(in_state(GameState::Playing)));
}

fn movement(
    time: Res<Time>,
    mut query: Query<(&mut Direction, &mut Transform), With<Boss>>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    let spawn_pos_y = window.height() * 0.5;

    for (mut logo, mut transform) in &mut query {
        match *logo {
            Direction::Up => transform.translation.y += 25. * time.delta_secs(),
            Direction::Down => transform.translation.y -= 25. * time.delta_secs(),
        }

        let y_diff = transform.translation.y - spawn_pos_y;
        if y_diff > 20. {
            *logo = Direction::Down;
        } else if y_diff < -20. {
            *logo = Direction::Up;
        }
    }
}

pub fn spawn_boss(commands: &mut Commands, chaset: &CharsetAsset, window: &Window, level: usize) {
    let spawn_pos_x = window.width() * 0.5;
    let spawn_pos_y = window.height() * 0.5;

    let e = commands
        .spawn((
            Sprite {
                image: chaset.texture.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: chaset.atlas.clone(),
                    index: BOSSES_GLYPH[level],
                }),
                ..Default::default()
            },
            Transform {
                translation: Vec3::new(spawn_pos_x, spawn_pos_y, 0.),
                rotation: Quat::IDENTITY,
                scale: Vec3::new(20.0, 20.0, 0.),
            },
            Boss {},
            Direction::Up,
            Pool {
                health: 1000.,
                max_health: 1000.,
                damage: 2.,
                god_mode: false,
            },
            GamePlayEntity,
        ))
        .id();

    match level {
        1 => commands.entity(e).insert(ShootPattern1),
        2 => commands.entity(e).insert(ShootPattern2),
        3 => commands.entity(e).insert((ShootPattern1, ShootPattern2)),
        _ => commands.entity(e).insert((ShootPattern1, ShootPattern2)),
    };
}

fn boss_shoot_pattern_1(
    mut commands: Commands,
    bosses: Query<(Entity, &Transform, &Pool), (With<Boss>, With<ShootPattern1>)>,
    time: Res<Time>,
    chaset: Res<CharsetAsset>,
) {
    let bullet_speed: f32 = 100.0;
    for (e, transform, pool) in &bosses {
        let rotation = Quat::from_rotation_z(time.elapsed_secs() * 2.);

        let dir = rotation.mul_vec3(transform.up().as_vec3());
        let dir2 = rotation.mul_vec3(transform.down().as_vec3());
        let dir3 = rotation.mul_vec3(transform.left().as_vec3());
        let dir4 = rotation.mul_vec3(transform.right().as_vec3());

        let mut directions = Vec::new();
        directions.push(dir);
        directions.push(dir2);
        directions.push(dir3);
        directions.push(dir4);

        shoot(
            &mut commands,
            e,
            directions,
            &chaset,
            transform,
            pool.damage,
            bullet_speed,
        );
    }
}

fn boss_shoot_pattern_2(
    mut commands: Commands,
    bosses: Query<(Entity, &Transform, &Pool), (With<Boss>, With<ShootPattern2>)>,
    chaset: Res<CharsetAsset>,
) {
    let bullet_speed: f32 = 100.0;
    for (e, transform, pool) in &bosses {
        let dir = transform.up().as_vec3();
        let dir2 = transform.down().as_vec3();
        let dir3 = transform.left().as_vec3();
        let dir4 = transform.right().as_vec3();

        let mut directions = Vec::new();
        directions.push(dir);
        directions.push(dir2);
        directions.push(dir3);
        directions.push(dir4);

        shoot(
            &mut commands,
            e,
            directions,
            &chaset,
            transform,
            pool.damage,
            bullet_speed,
        );
    }
}

fn shoot(
    commands: &mut Commands,
    entity: Entity,
    dir: Vec<Vec3>,
    chaset: &CharsetAsset,
    transform: &Transform,
    damage: f32,
    bullet_speed: f32,
) {
    for dir in dir.iter() {
        spawn_bullet(
            commands,
            &chaset,
            *dir,
            transform.translation,
            entity,
            bullet_speed,
            damage,
        );
    }
}
