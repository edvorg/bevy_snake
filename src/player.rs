use std::time::Duration;
use bevy::color::palettes::css::WHITE;
use bevy::prelude::*;

const MOVEMENT_INTERVAL: Duration = Duration::from_millis(500);

#[derive(Component)]
pub struct Player;

#[derive(Component)]
struct Direction {
    direction: IVec2,
}

#[derive(Resource)]
struct MovementTimer {
    timer: Timer,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MovementTimer {
            timer: Timer::new(MOVEMENT_INTERVAL, TimerMode::Repeating),
        });
        app.add_systems(Update, input);
        app.add_systems(Update, movement);
        app.add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // Sphere
    commands
        .spawn((
            Player,
            PbrBundle {
                mesh: meshes.add(Mesh::from(Sphere::default())),
                material: materials.add(StandardMaterial {
                    base_color: Color::srgb(6.5, 6.5, 6.5),
                    unlit: true,
                    ..Default::default()
                }),
                ..Default::default()
            },
            Direction {
                direction: IVec2::ZERO,
            },
        ))
        .with_children(|children| {
            children.spawn(PointLightBundle {
                point_light: PointLight {
                    shadows_enabled: true,
                    intensity: 10_000_0.,
                    range: 100.0,
                    shadow_depth_bias: 0.1,
                    radius: 0.5,
                    color: WHITE.into(),
                    ..Default::default()
                },
                ..Default::default()
            });
        });
}

fn input(
    mut query: Query<&mut Direction, With<Player>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut exit: EventWriter<AppExit>,
) {
    if keyboard.pressed(KeyCode::ArrowLeft) {
        for mut direction in query.iter_mut() {
            direction.direction = IVec2::new(-1, 0);
        }
    }
    if keyboard.pressed(KeyCode::ArrowRight) {
        for mut direction in query.iter_mut() {
            direction.direction = IVec2::new(1, 0);
        }
    }
    if keyboard.pressed(KeyCode::ArrowUp) {
        for mut direction in query.iter_mut() {
            direction.direction = IVec2::new(0, -1);
        }
    }
    if keyboard.pressed(KeyCode::ArrowDown) {
        for mut direction in query.iter_mut() {
            direction.direction = IVec2::new(0, 1);
        }
    }
    if keyboard.pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }
}

fn movement(
    mut query: Query<(&mut Transform, &Direction)>,
    mut timer: ResMut<MovementTimer>,
    time: Res<Time>,
) {
    if timer.timer.tick(time.delta()).just_finished() {
        for (mut transform, direction) in query.iter_mut() {
            transform.translation += Vec3::new(
                direction.direction.x as f32,
                0.0,
                direction.direction.y as f32,
            );
        }
    }
}
