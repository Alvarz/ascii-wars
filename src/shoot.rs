use bevy::{prelude::*, text::cosmic_text::ttf_parser::kern};

use crate::{assets::CharsetAsset, player::ApplyMove, GameState};

#[derive(Component)]
pub struct WantToShoot {
    pub dir: Vec3,
}

#[derive(Component)]
pub struct Bullet {
    pub dir: Vec3,
    pub lifetime: f32,
}

const BULLET_SPEED: f32 = 5.0;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, shoot.run_if(in_state(GameState::Playing)));
    app.add_systems(Update, bullet_movement.run_if(in_state(GameState::Playing)));
}

fn shoot(
    mut commands: Commands,
    shooters: Query<(Entity, &Transform, &WantToShoot), With<WantToShoot>>,
    chaset: Res<CharsetAsset>,
) {
    for (e, transform, shooter) in &shooters {
        let direction = shooter.dir - transform.translation;

        spawn_player_bullet(&mut commands, &chaset, direction, transform.translation);

        commands.entity(e).remove::<WantToShoot>();
    }
}

fn spawn_player_bullet(commands: &mut Commands, chaset: &CharsetAsset, dir: Vec3, position: Vec3) {
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
        Bullet { dir, lifetime: 3.0 },
    ));
}

fn bullet_movement(
    mut commands: Commands,
    time: Res<Time>,
    mut bullets: Query<(Entity, &mut Transform, &mut Bullet), With<Bullet>>,
) {
    for (e, mut transform, mut bullet) in &mut bullets.iter_mut() {
        transform.translation += bullet.dir * BULLET_SPEED * time.delta_secs();
        bullet.lifetime -= time.delta_secs();

        if bullet.lifetime <= 0. {
            info!("removed bullet");
            commands.entity(e).despawn();
        }
    }
}
