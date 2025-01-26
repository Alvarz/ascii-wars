use std::{thread, time::Duration};

use bevy::{prelude::*, utils::info, window::PrimaryWindow};

use crate::{
    assets::CharsetAsset,
    game::{GamePlayEntity, GameState, Pool},
    player::Player,
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
struct ShootPattern1 {
    bullet_speed: f32,
    spawn_count: i32,
    rotation_speed: f32,
    fire_rate: f32,
    bullet_size: f32,
    bullet_glyph: usize,
}

#[derive(Component)]
struct ShootPattern2 {
    bullet_speed: f32,
    spawn_count: i32,
    rotation_speed: f32,
    fire_rate: f32,
    bullet_size: f32,
    bullet_glyph: usize,
}

#[derive(Component)]
struct ShootPattern3 {
    bullet_speed: f32,
    spawn_count: i32,
    rotation_speed: f32,
    fire_rate: f32,
    bullet_size: f32,
    bullet_glyph: usize,
}

#[derive(Component)]
struct ShootPattern4 {
    bullet_speed: f32,
    spawn_count: i32,
    rotation_speed: f32,
    fire_rate: f32,
    bullet_size: f32,
    bullet_glyph: usize,
}

#[derive(Component)]
struct ShootPatternDirectShoot {
    bullet_speed: f32,
    spawn_count: i32,
    fire_rate: f32,
    bullet_size: f32,
    bullet_glyph: usize,
    bullet_frequency: u64,
}

#[derive(Resource)]
struct Pattern1Timer {
    timer: Timer,
}

#[derive(Resource)]
struct Pattern2Timer {
    timer: Timer,
}

#[derive(Resource)]
struct Pattern3Timer {
    timer: Timer,
}

#[derive(Resource)]
struct Pattern4Timer {
    timer: Timer,
}

#[derive(Resource)]
struct ShootPatternDirectShootTimer {
    timer: Timer,
}
const BOSSES_GLYPH: [usize; 10] = [
    '#' as usize, // The Sentinel
    '$' as usize, // The Hoarder
    '%' as usize, //  The Watcher
    '@' as usize, // The Infernal Imp
    '&' as usize, // The Spiral Beast
    '+' as usize, // The Guardian of Order
    '*' as usize, // The Swarm King
    'O' as usize, // The Celestial Tyrant
    '=' as usize, // The Clockwork Leviathan
    '~' as usize, // The Chaos Reaper
];

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(ShootPatternDirectShootTimer {
        timer: Timer::from_seconds(0.0, TimerMode::Repeating),
    });
    app.insert_resource(Pattern4Timer {
        timer: Timer::from_seconds(0.0, TimerMode::Repeating),
    });
    app.insert_resource(Pattern3Timer {
        timer: Timer::from_seconds(0.0, TimerMode::Repeating),
    });
    app.insert_resource(Pattern2Timer {
        timer: Timer::from_seconds(0.0, TimerMode::Repeating),
    });

    app.insert_resource(Pattern1Timer {
        timer: Timer::from_seconds(0.0, TimerMode::Repeating),
    });
    app.add_systems(
        Update,
        boss_shoot_pattern_1.run_if(in_state(GameState::Playing)),
    );

    app.add_systems(
        Update,
        boss_shoot_pattern_2.run_if(in_state(GameState::Playing)),
    );

    app.add_systems(
        Update,
        boss_shoot_pattern_3.run_if(in_state(GameState::Playing)),
    );

    app.add_systems(
        Update,
        boss_shoot_pattern_4.run_if(in_state(GameState::Playing)),
    );

    app.add_systems(
        Update,
        boss_shoot_pattern_direct_shoot.run_if(in_state(GameState::Playing)),
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
                    index: BOSSES_GLYPH[level - 1],
                }),
                ..Default::default()
            },
            Transform {
                translation: Vec3::new(spawn_pos_x, spawn_pos_y, 0.),
                rotation: Quat::IDENTITY,
                scale: Vec3::new(2.0, 2.0, 0.),
            },
            Boss {},
            // Direction::Up,
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
        // 1 => the_sentinel(commands, e),
        // 2 => the_hoarder(commands, e),
        // 3 => the_watcher(commands, e),
        // 4 => the_infernal_imp(commands, e),
        // 5 => the_spiral_beast(commands, e),
        // 6 => the_guardian_order(commands, e),
        // 7 => the_swarm_king(commands, e),
        // 8 => the_celestial_tyrant(commands, e),
        // 9 => the_clockwork_leviathan(commands, e),
        // 10 => the_chaos_reaper(commands, e),
        _ => the_celestial_tyrant(commands, e),
    };
}

fn boss_shoot_pattern_1(
    mut commands: Commands,
    bosses: Query<(Entity, &Transform, &Pool, &ShootPattern1), (With<Boss>, With<ShootPattern1>)>,
    time: Res<Time>,
    mut pattern_timer: ResMut<Pattern1Timer>,
    charset: Res<CharsetAsset>,
) {
    if pattern_timer.timer.tick(time.delta()).just_finished() {
        for (e, transform, pool, pattern) in &bosses {
            pattern_timer.timer = Timer::from_seconds(pattern.fire_rate, TimerMode::Repeating);
            process_pattern(
                &mut commands,
                &e,
                &charset,
                transform,
                pool,
                *time,
                pattern.bullet_speed,
                pattern.rotation_speed,
                pattern.spawn_count,
                pattern.bullet_size,
                pattern.bullet_glyph,
            );
        }
    }
}

fn boss_shoot_pattern_2(
    mut commands: Commands,
    bosses: Query<(Entity, &Transform, &Pool, &ShootPattern2), (With<Boss>, With<ShootPattern2>)>,
    charset: Res<CharsetAsset>,
    mut pattern_timer: ResMut<Pattern2Timer>,
    time: Res<Time>,
) {
    if pattern_timer.timer.tick(time.delta()).just_finished() {
        for (e, transform, pool, pattern) in &bosses {
            pattern_timer.timer = Timer::from_seconds(pattern.fire_rate, TimerMode::Repeating);
            process_pattern(
                &mut commands,
                &e,
                &charset,
                transform,
                pool,
                *time,
                pattern.bullet_speed,
                pattern.rotation_speed,
                pattern.spawn_count,
                pattern.bullet_size,
                pattern.bullet_glyph,
            );
        }
    }
}

fn boss_shoot_pattern_3(
    mut commands: Commands,
    bosses: Query<(Entity, &Transform, &Pool, &ShootPattern3), (With<Boss>, With<ShootPattern3>)>,
    time: Res<Time>,
    charset: Res<CharsetAsset>,
    mut pattern_timer: ResMut<Pattern3Timer>,
) {
    if pattern_timer.timer.tick(time.delta()).just_finished() {
        for (e, transform, pool, pattern) in &bosses {
            pattern_timer.timer = Timer::from_seconds(pattern.fire_rate, TimerMode::Repeating);

            process_pattern(
                &mut commands,
                &e,
                &charset,
                transform,
                pool,
                *time,
                pattern.bullet_speed,
                pattern.rotation_speed,
                pattern.spawn_count,
                pattern.bullet_size,
                pattern.bullet_glyph,
            );
        }
    }
}

fn boss_shoot_pattern_4(
    mut commands: Commands,
    bosses: Query<(Entity, &Transform, &Pool, &ShootPattern4), (With<Boss>, With<ShootPattern4>)>,
    time: Res<Time>,
    charset: Res<CharsetAsset>,
    mut pattern_timer: ResMut<Pattern4Timer>,
) {
    if pattern_timer.timer.tick(time.delta()).just_finished() {
        for (e, transform, pool, pattern) in &bosses {
            pattern_timer.timer = Timer::from_seconds(pattern.fire_rate, TimerMode::Repeating);

            process_pattern(
                &mut commands,
                &e,
                &charset,
                transform,
                pool,
                *time,
                pattern.bullet_speed,
                pattern.rotation_speed,
                pattern.spawn_count,
                pattern.bullet_size,
                pattern.bullet_glyph,
            );
        }
    }
}

fn boss_shoot_pattern_direct_shoot(
    mut commands: Commands,
    bosses: Query<
        (Entity, &Transform, &Pool, &ShootPatternDirectShoot),
        (With<Boss>, With<ShootPatternDirectShoot>),
    >,
    time: Res<Time>,
    charset: Res<CharsetAsset>,
    mut pattern_timer: ResMut<ShootPatternDirectShootTimer>,
    players: Query<&Transform, With<Player>>,
) {
    if pattern_timer.timer.tick(time.delta()).just_finished() {
        for (e, transform, pool, pattern) in &bosses {
            pattern_timer.timer = Timer::from_seconds(pattern.fire_rate, TimerMode::Repeating);

            for player_transform in &players {
                let mut directions = Vec::new();
                for _ in 0..pattern.spawn_count {
                    let dir = (player_transform.translation - transform.translation).normalize();
                    directions.push(dir);
                }

                shoot(
                    &mut &mut commands,
                    e,
                    directions,
                    &charset,
                    transform,
                    pool.damage,
                    pattern.bullet_speed,
                    pattern.bullet_size,
                    pattern.bullet_glyph,
                );
            }
        }
    }
}

fn process_pattern(
    commands: &mut Commands,
    e: &Entity,
    charset: &CharsetAsset,
    transform: &Transform,
    pool: &Pool,
    time: Time,
    bullet_speed: f32,
    rotation_speed: f32,
    spawn_count: i32,
    bullet_size: f32,
    bullet_glyph: usize,
) {
    let rotation = Quat::from_rotation_z(time.elapsed_secs() * rotation_speed);

    let mut directions = Vec::new();

    for i in 0..spawn_count {
        let angle = (i as f32 / spawn_count as f32) * 2.0 * std::f32::consts::PI;
        let dir = rotation * Vec3::new(angle.cos(), angle.sin(), 0.0);
        directions.push(dir);
    }

    shoot(
        commands,
        *e,
        directions,
        charset,
        transform,
        pool.damage,
        bullet_speed,
        bullet_size,
        bullet_glyph,
    );
}

fn shoot(
    commands: &mut Commands,
    entity: Entity,
    dir: Vec<Vec3>,
    chaset: &CharsetAsset,
    transform: &Transform,
    damage: f32,
    bullet_speed: f32,
    size: f32,
    bullet_glyph: usize,
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
            size,
            bullet_glyph,
        );
    }
}

fn the_sentinel(commands: &mut Commands, e: Entity) {
    commands.entity(e).insert((
        ShootPattern1 {
            bullet_speed: 100.,
            spawn_count: 4,
            rotation_speed: 0.,
            fire_rate: 0.8,
            bullet_size: 3.0,
            bullet_glyph: '.' as usize,
        },
        ShootPatternDirectShoot {
            bullet_speed: 100.,
            spawn_count: 1,
            fire_rate: 1.,
            bullet_size: 3.0,
            bullet_glyph: '|' as usize,
            bullet_frequency: 0.2 as u64,
        },
    ));
}

fn the_watcher(commands: &mut Commands, e: Entity) {
    commands.entity(e).insert((
        ShootPattern1 {
            bullet_speed: 100.,
            spawn_count: 64,
            rotation_speed: 0.,
            fire_rate: 1.,
            bullet_size: 1.0,
            bullet_glyph: 'o' as usize,
        },
        ShootPatternDirectShoot {
            bullet_speed: 300.,
            spawn_count: 1,
            fire_rate: 3.,
            bullet_size: 3.0,
            bullet_glyph: '|' as usize,
            bullet_frequency: 0.2 as u64,
        },
    ));
}

fn the_infernal_imp(commands: &mut Commands, e: Entity) {
    commands.entity(e).insert((
        ShootPattern1 {
            bullet_speed: 300.,
            spawn_count: 4,
            rotation_speed: 0.3,
            fire_rate: 0.,
            bullet_size: 1.0,
            bullet_glyph: '~' as usize,
        },
        ShootPatternDirectShoot {
            bullet_speed: 250.,
            spawn_count: 1,
            fire_rate: 2.,
            bullet_size: 2.0,
            bullet_glyph: 'O' as usize,
            bullet_frequency: 0.2 as u64,
        },
        ShootPattern2 {
            bullet_speed: 100.,
            spawn_count: 32,
            rotation_speed: 0.,
            fire_rate: 2.,
            bullet_size: 1.0,
            bullet_glyph: 'o' as usize,
        },
    ));
}

fn the_spiral_beast(commands: &mut Commands, e: Entity) {
    commands.entity(e).insert((
        ShootPattern1 {
            bullet_speed: 250.,
            spawn_count: 16,
            rotation_speed: 0.8,
            fire_rate: 0.2,
            bullet_size: 1.5,
            bullet_glyph: '0' as usize,
        },
        ShootPattern2 {
            bullet_speed: 100.,
            spawn_count: 6,
            rotation_speed: 0.8,
            fire_rate: 0.05,
            bullet_size: 1.0,
            bullet_glyph: 'x' as usize,
        },
        ShootPattern3 {
            bullet_speed: 80.,
            spawn_count: 64,
            rotation_speed: 0.,
            fire_rate: 5.,
            bullet_size: 2.0,
            bullet_glyph: '|' as usize,
        },
    ));
}

fn the_guardian_order(commands: &mut Commands, e: Entity) {
    commands.entity(e).insert((
        ShootPattern1 {
            bullet_speed: 300.,
            spawn_count: 4,
            rotation_speed: 0.,
            fire_rate: 0.4,
            bullet_size: 1.5,
            bullet_glyph: 'x' as usize,
        },
        ShootPatternDirectShoot {
            bullet_speed: 80.,
            spawn_count: 1,
            fire_rate: 2.,
            bullet_size: 2.0,
            bullet_glyph: 'o' as usize,
            bullet_frequency: 0.2 as u64,
        },
        ShootPattern2 {
            bullet_speed: 300.,
            spawn_count: 8,
            rotation_speed: 0.3,
            fire_rate: 0.1,
            bullet_size: 1.,
            bullet_glyph: '|' as usize,
        },
        ShootPattern3 {
            bullet_speed: 50.,
            spawn_count: 64,
            rotation_speed: 0.3,
            fire_rate: 3.5,
            bullet_size: 1.,
            bullet_glyph: '#' as usize,
        },
    ));
}

fn the_swarm_king(commands: &mut Commands, e: Entity) {
    commands.entity(e).insert((
        ShootPattern1 {
            bullet_speed: 300.,
            spawn_count: 5,
            rotation_speed: 0.,
            fire_rate: 0.3,
            bullet_size: 1.3,
            bullet_glyph: 'x' as usize,
        },
        ShootPattern2 {
            bullet_speed: 100.,
            spawn_count: 64,
            rotation_speed: 0.,
            fire_rate: 2.,
            bullet_size: 1.,
            bullet_glyph: 'O' as usize,
        },
        ShootPattern3 {
            bullet_speed: 150.,
            spawn_count: 16,
            rotation_speed: 2.,
            fire_rate: 0.5,
            bullet_size: 1.,
            bullet_glyph: '%' as usize,
        },
        ShootPatternDirectShoot {
            bullet_speed: 450.,
            spawn_count: 1,
            fire_rate: 1.,
            bullet_size: 3.0,
            bullet_glyph: '.' as usize,
            bullet_frequency: 0.2 as u64,
        },
    ));
}

fn the_celestial_tyrant(commands: &mut Commands, e: Entity) {
    commands.entity(e).insert((
        ShootPattern1 {
            bullet_speed: 300.,
            spawn_count: 8,
            rotation_speed: 0.0,
            fire_rate: 0.2,
            bullet_size: 1.3,
            bullet_glyph: '|' as usize,
        },
        ShootPattern2 {
            bullet_speed: 600.,
            spawn_count: 64,
            rotation_speed: 0.0,
            fire_rate: 5.,
            bullet_size: 1.3,
            bullet_glyph: 'o' as usize,
        },
        ShootPatternDirectShoot {
            bullet_speed: 200.,
            spawn_count: 1,
            fire_rate: 0.4,
            bullet_size: 2.0,
            bullet_glyph: 'x' as usize,
            bullet_frequency: 0.2 as u64,
        },
        ShootPattern3 {
            bullet_speed: 250.,
            spawn_count: 5,
            rotation_speed: 0.5,
            fire_rate: 0.1,
            bullet_size: 1.5,
            bullet_glyph: '*' as usize,
        },
        ShootPattern4 {
            bullet_speed: 250.,
            spawn_count: 5,
            rotation_speed: -0.5,
            fire_rate: 0.1,
            bullet_size: 1.5,
            bullet_glyph: '*' as usize,
        },
    ));
}
