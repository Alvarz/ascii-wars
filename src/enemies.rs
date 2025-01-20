use bevy::{prelude::*, window::PrimaryWindow};

use crate::{assets::CharsetAsset, shoot::WantToShoot, GameState};

#[derive(Component)]
pub struct Boss;

#[derive(Component)]
enum Direction {
    Up,
    Down,
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::NewGame), spawn_boss);
    app.add_systems(Update, boss_shoot.run_if(in_state(GameState::Playing)));
    app.add_systems(FixedUpdate, movement.run_if(in_state(GameState::Playing)));
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

fn spawn_boss(
    mut commands: Commands,
    chaset: Res<CharsetAsset>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    let spawn_pos_x = window.width() * 0.5;
    let spawn_pos_y = window.height() * 0.5;

    commands.spawn((
        Sprite {
            image: chaset.texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: chaset.atlas.clone(),
                index: '@' as usize,
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
    ));
}

fn boss_shoot(
    mut commands: Commands,
    boss: Single<(Entity, &Transform), With<Boss>>,
    time: Res<Time>,
) {
    let e = boss.0;
    let transform = boss.1;
    //transform.rotation = Quat::from_rotation_z(time.elapsed_secs() * 2.);
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
    commands.entity(e).insert(WantToShoot {
        dir: directions,
        entity: e,
    });
}
