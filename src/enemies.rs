use bevy::{prelude::*, window::PrimaryWindow};

use crate::{assets::CharsetAsset, shoot::WantToShoot, GameState};

#[derive(Component)]
pub struct Boss;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::NewGame), spawn_boss);
    app.add_systems(Update, rotate_boss.run_if(in_state(GameState::Playing)));
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
    ));
}

fn rotate_boss(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform), With<Boss>>,
    time: Res<Time>,
) {
    for (e, mut transform) in query.iter_mut() {
        transform.rotation = Quat::from_rotation_z(time.elapsed_secs() * 2.);

        let dir = transform.up().as_vec3();
        let dir2 = transform.down().as_vec3();

        let dir3 = transform.right().as_vec3();
        let dir4 = transform.left().as_vec3();

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
}
