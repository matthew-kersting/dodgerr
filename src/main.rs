use bevy::app::AppExit;
use bevy::prelude::*;
use rand::Rng;

const ARENA_SIZE: Vec2 = Vec2::new(800.0, 600.0);
const PLAYER_RADIUS: f32 = 15.0;
const PLAYER_SPEED: f32 = 200.0;
const ENEMY_SIZE: Vec2 = Vec2::new(30.0, 30.0);
const PROJECTILE_SIZE: Vec2 = Vec2::new(5.0, 10.0);
const PROJECTILE_BASE_SPEED: f32 = 200.0;
const ENEMY_BASE_SPEED: f32 = 200.0;
const PROJECTILE_SPEED_PER_LEVEL: f32 = 0.2;
const ENEMY_SPEED_PER_LEVEL: f32 = 0.1;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct MovingEnemy {
    direction: Vec2,
}

#[derive(Component)]
struct Projectile {
    direction: Vec3,
}

#[derive(Component)]
struct LevelText;

#[derive(Resource)]
struct Score(f32);

#[derive(Resource)]
struct Level {
    value: u32,
    timer: Timer,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Avoid the Projectiles!".to_string(),
                resolution: (ARENA_SIZE.x, ARENA_SIZE.y).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(Score(0.0))
        .insert_resource(Level {
            value: 1,
            timer: Timer::from_seconds(20.0, TimerMode::Repeating),
        })
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                player_movement,
                enemy_movement,
                spawn_projectiles,
                projectile_movement,
                collision_detection,
                score_timer,
                level_timer,
            ),
        )
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::ZERO,
                scale: Vec3::splat(PLAYER_RADIUS * 2.0),
                ..default()
            },
            sprite: Sprite {
                color: Color::CYAN,
                ..default()
            },
            ..default()
        },
        Player,
    ));

    let positions = vec![
        Vec3::new(0.0, ARENA_SIZE.y / 2.0 - 30.0, 0.0),  // Top
        Vec3::new(0.0, -ARENA_SIZE.y / 2.0 + 30.0, 0.0), // Bottom
        Vec3::new(-ARENA_SIZE.x / 2.0 + 30.0, 0.0, 0.0), // Left
        Vec3::new(ARENA_SIZE.x / 2.0 - 30.0, 0.0, 0.0),  // Right
    ];

    let mut rng = rand::thread_rng();

    for pos in positions {
        let is_vertical = pos.x.abs() < 1.0;
        let direction = if is_vertical {
            Vec2::new(rng.gen_range(-1.0..1.0), 0.0)
        } else {
            Vec2::new(0.0, rng.gen_range(-1.0..1.0))
        };

        commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: pos,
                    scale: Vec3::new(ENEMY_SIZE.x, ENEMY_SIZE.y, 1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: Color::RED,
                    ..default()
                },
                ..default()
            },
            Enemy,
            MovingEnemy { direction },
        ));
    }

    commands.spawn((
        TextBundle::from_section(
            "Level: 1",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 30.0,
                color: Color::YELLOW,
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
        LevelText,
    ));
}

fn player_movement(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let mut transform = query.single_mut();
    let mut direction = Vec3::ZERO;

    if keys.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }
    if keys.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }
    if keys.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }
    if keys.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }

    transform.translation += direction.normalize_or_zero() * PLAYER_SPEED * time.delta_seconds();
}

fn enemy_movement(
    time: Res<Time>,
    level: Res<Level>,
    mut query: Query<(&mut Transform, &mut MovingEnemy)>,
) {
    let speed = ENEMY_BASE_SPEED * (1.0 + ENEMY_SPEED_PER_LEVEL * level.value as f32);

    for (mut transform, mut moving) in query.iter_mut() {
        transform.translation.x += moving.direction.x * speed * time.delta_seconds();
        transform.translation.y += moving.direction.y * speed * time.delta_seconds();

        if transform.translation.x.abs() > ARENA_SIZE.x / 2.0 - 50.0 {
            moving.direction.x *= -1.0;
        }
        if transform.translation.y.abs() > ARENA_SIZE.y / 2.0 - 50.0 {
            moving.direction.y *= -1.0;
        }

        if rand::random::<f32>() < 0.001 {
            moving.direction *= -1.0;
        }
    }
}

fn spawn_projectiles(
    mut commands: Commands,
    time: Res<Time>,
    enemies: Query<&Transform, With<Enemy>>,
    mut timer: Local<f32>,
) {
    *timer += time.delta_seconds();
    if *timer > 0.6 {
        *timer = 0.0;
        let mut rng = rand::thread_rng();
        let idx = rng.gen_range(0..4);
        if let Some(enemy_transform) = enemies.iter().nth(idx) {
            let pos = enemy_transform.translation;
            let dir = if pos.y > ARENA_SIZE.y / 4.0 {
                Vec3::new(0.0, -1.0, 0.0) // top -> down
            } else if pos.y < -ARENA_SIZE.y / 4.0 {
                Vec3::new(0.0, 1.0, 0.0) // bottom -> up
            } else if pos.x < -ARENA_SIZE.x / 4.0 {
                Vec3::new(1.0, 0.0, 0.0) // left -> right
            } else {
                Vec3::new(-1.0, 0.0, 0.0) // right -> left
            };

            commands.spawn((
                SpriteBundle {
                    transform: Transform {
                        translation: pos,
                        scale: Vec3::new(PROJECTILE_SIZE.x, PROJECTILE_SIZE.y, 1.0),
                        ..default()
                    },
                    sprite: Sprite {
                        color: Color::WHITE,
                        ..default()
                    },
                    ..default()
                },
                Projectile { direction: dir },
            ));
        }
    }
}

fn projectile_movement(
    time: Res<Time>,
    level: Res<Level>,
    mut query: Query<(&mut Transform, &Projectile)>,
) {
    let speed = PROJECTILE_BASE_SPEED * (1.0 + PROJECTILE_SPEED_PER_LEVEL * level.value as f32);

    for (mut transform, projectile) in &mut query {
        transform.translation += projectile.direction * speed * time.delta_seconds();
    }
}

fn collision_detection(
    mut exit: EventWriter<AppExit>,
    projectile_query: Query<&Transform, With<Projectile>>,
    player_query: Query<&Transform, With<Player>>,
) {
    let player_pos = player_query.single().translation;

    for transform in &projectile_query {
        if transform.translation.distance(player_pos) < PLAYER_RADIUS {
            println!("YOU LOSE!");
            exit.send(AppExit);
            break;
        }
    }
}

fn score_timer(time: Res<Time>, mut score: ResMut<Score>) {
    score.0 += time.delta_seconds();
    println!("Score: {:.2}", score.0);
}

fn level_timer(
    time: Res<Time>,
    mut level: ResMut<Level>,
    mut query: Query<&mut Text, With<LevelText>>,
) {
    if level.timer.tick(time.delta()).just_finished() {
        level.value += 1;
        println!("Increasing Power Levels! Level = {}", level.value);

        if let Ok(mut text) = query.get_single_mut() {
            text.sections[0].value = format!("Level: {}", level.value);
        }
    }
}
