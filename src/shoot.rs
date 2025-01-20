use bevy::{
    math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume},
    prelude::*,
};

use crate::{assets::CharsetAsset, camera::MainCamera, enemies::Boss, player::Player, GameState};

#[derive(Component)]
pub struct WantToShoot {
    pub dir: Vec<Vec3>,
    pub entity: Entity,
}

#[derive(Component)]
pub struct Bullet {
    pub dir: Vec3,
    pub lifetime: f32,
    pub owner: Entity,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Collision {
    Left,
    Right,
    Top,
    Bottom,
}

const BULLET_SPEED: f32 = 100.0;
const PLAYER_BULLET_SPEED: f32 = 500.0;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, shoot.run_if(in_state(GameState::Playing)));
    app.add_systems(
        FixedUpdate,
        bullet_movement.run_if(in_state(GameState::Playing)),
    );
    app.add_systems(
        Update,
        check_for_collisions.run_if(in_state(GameState::Playing)),
    );
}

fn shoot(
    mut commands: Commands,
    shooters: Query<(Entity, &Transform, &WantToShoot), With<Player>>,
    shooters_bosses: Query<(Entity, &Transform, &WantToShoot), With<Boss>>,
    chaset: Res<CharsetAsset>,
) {
    for (e, transform, shooter) in &shooters {
        for dir in shooter.dir.iter() {
            let direction = dir - transform.translation;

            spawn_player_bullet(
                &mut commands,
                &chaset,
                direction.normalize(),
                transform.translation,
                shooter.entity,
            );

            commands.entity(e).remove::<WantToShoot>();
        }
    }

    for (e, transform, shooter) in &shooters_bosses {
        for dir in shooter.dir.iter() {
            // let direction = shooter.dir - transform.translation;

            spawn_player_bullet(
                &mut commands,
                &chaset,
                *dir,
                transform.translation,
                shooter.entity,
            );

            commands.entity(e).remove::<WantToShoot>();
        }
    }
}

fn spawn_player_bullet(
    commands: &mut Commands,
    chaset: &CharsetAsset,
    dir: Vec3,
    position: Vec3,
    owner: Entity,
) {
    commands.spawn((
        Sprite {
            image: chaset.texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: chaset.atlas.clone(),
                index: '*' as usize,
            }),

            ..Default::default()
        },
        Transform::from_xyz(position.x, position.y, position.z),
        Bullet {
            dir,
            lifetime: 10.0,
            owner,
        },
    ));
}

fn bullet_movement(
    mut commands: Commands,
    time: Res<Time>,
    mut bullets: Query<(Entity, &mut Transform, &mut Bullet), With<Bullet>>,
    player: Single<Entity, With<Player>>,
) {
    for (e, mut transform, mut bullet) in &mut bullets.iter_mut() {
        if e == *player {
            transform.translation += bullet.dir * PLAYER_BULLET_SPEED * time.delta_secs();
        } else {
            transform.translation += bullet.dir * BULLET_SPEED * time.delta_secs();
        }

        bullet.lifetime -= time.delta_secs();

        if bullet.lifetime <= 0. {
            //     info!("removed bullet");
            commands.entity(e).despawn();
        }
    }
}

fn check_for_collisions(
    mut commands: Commands,
    bullets: Query<(Entity, &Transform, &Bullet), With<Bullet>>,
    no_bullets: Query<(Entity, &Transform), Without<Bullet>>,
    camera: Single<(Entity, &MainCamera)>,
) {
    for (bullet_e, bullet_transform, bullet) in &bullets {
        for (e, transform) in &no_bullets {
            let collision = check_collision(
                BoundingCircle::new(bullet_transform.translation.truncate(), 8. * 0.5),
                Aabb2d::new(
                    transform.translation.truncate(),
                    transform.scale.truncate() * 0.5,
                ),
            );

            if let Some(collision) = collision {
                // info!("collide with {:?} on {:?}", e, collision);
                // println!("scale {:?}", transform.scale);
                if e != bullet.owner && e != camera.0 {
                    //     commands.entity(e).despawn();
                    //     commands.entity(bullet_e).despawn();
                }
            }
        }
    }
}

fn check_collision(bullet: BoundingCircle, bounding_box: Aabb2d) -> Option<Collision> {
    if !bullet.intersects(&bounding_box) {
        return None;
    }

    let closest = bounding_box.closest_point(bullet.center());
    let offset = bullet.center() - closest;
    let side = if offset.x.abs() > offset.y.abs() {
        if offset.x < 0. {
            Collision::Left
        } else {
            Collision::Right
        }
    } else if offset.y > 0. {
        Collision::Top
    } else {
        Collision::Bottom
    };

    Some(side)
}
