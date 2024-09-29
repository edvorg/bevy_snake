use crate::level::TreatEatenEvent;
use bevy::color::palettes::css::WHITE;
use bevy::prelude::*;
use bevy::utils::HashMap;
use std::mem::swap;
use std::time::Duration;

const MOVEMENT_INTERVAL: Duration = Duration::from_millis(250);

#[derive(Component)]
pub struct SnakeHead;

#[derive(Component)]
pub struct SnakeTail;

#[derive(Component)]
pub struct SnakeLink {
    previous: Option<Entity>,
}

#[derive(Component)]
struct Velocity {
    velocity: Vec2,
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
        app.add_systems(Update, (move_links, grow_links).chain());
        app.add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let link0 = commands
        .spawn((
            SnakeLink { previous: None },
            SnakeTail,
            PbrBundle {
                mesh: meshes.add(Mesh::from(Sphere::default())),
                material: materials.add(StandardMaterial {
                    base_color: Color::srgb(6.5, 6.5, 6.5),
                    unlit: true,
                    ..Default::default()
                }),
                ..Default::default()
            },
            Velocity {
                velocity: Vec2::ZERO,
            },
        ))
        .with_children(|children| {
            children.spawn(PointLightBundle {
                point_light: PointLight {
                    shadows_enabled: true,
                    intensity: 100_000.,
                    range: 100.0,
                    shadow_depth_bias: 0.1,
                    radius: 0.5,
                    color: WHITE.into(),
                    ..Default::default()
                },
                ..Default::default()
            });
        })
        .id();

    let link1 = commands
        .spawn((
            SnakeLink {
                previous: Some(link0),
            },
            PbrBundle {
                mesh: meshes.add(Mesh::from(Sphere::default())),
                material: materials.add(StandardMaterial {
                    base_color: Color::srgb(6.5, 6.5, 6.5),
                    unlit: true,
                    ..Default::default()
                }),
                ..Default::default()
            },
            Velocity {
                velocity: Vec2::ZERO,
            },
        ))
        .with_children(|children| {
            children.spawn(PointLightBundle {
                point_light: PointLight {
                    shadows_enabled: true,
                    intensity: 100_000.,
                    range: 100.0,
                    shadow_depth_bias: 0.1,
                    radius: 0.5,
                    color: WHITE.into(),
                    ..Default::default()
                },
                ..Default::default()
            });
        })
        .id();

    let link2 = commands
        .spawn((
            SnakeLink {
                previous: Some(link1),
            },
            PbrBundle {
                mesh: meshes.add(Mesh::from(Sphere::default())),
                material: materials.add(StandardMaterial {
                    base_color: Color::srgb(6.5, 6.5, 6.5),
                    unlit: true,
                    ..Default::default()
                }),
                ..Default::default()
            },
            Velocity {
                velocity: Vec2::ZERO,
            },
        ))
        .with_children(|children| {
            children.spawn(PointLightBundle {
                point_light: PointLight {
                    shadows_enabled: true,
                    intensity: 100_000.,
                    range: 100.0,
                    shadow_depth_bias: 0.1,
                    radius: 0.5,
                    color: WHITE.into(),
                    ..Default::default()
                },
                ..Default::default()
            });
        })
        .id();

    commands
        .spawn((
            SnakeHead,
            SnakeLink {
                previous: Some(link2),
            },
            PbrBundle {
                mesh: meshes.add(Mesh::from(Sphere::default())),
                material: materials.add(StandardMaterial {
                    base_color: Color::srgb(6.5, 6.5, 6.5),
                    unlit: true,
                    ..Default::default()
                }),
                ..Default::default()
            },
            Velocity {
                velocity: Vec2::ZERO,
            },
        ))
        .with_children(|children| {
            children.spawn(PointLightBundle {
                point_light: PointLight {
                    shadows_enabled: true,
                    intensity: 100_000.,
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
    mut query: Query<&mut Velocity, With<SnakeHead>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut exit: EventWriter<AppExit>,
) {
    if keyboard.pressed(KeyCode::ArrowLeft) {
        for mut velocity in query.iter_mut() {
            velocity.velocity = Vec2::new(1.0, 0.0);
        }
    }
    if keyboard.pressed(KeyCode::ArrowRight) {
        for mut velocity in query.iter_mut() {
            velocity.velocity = Vec2::new(-1.0, 0.0);
        }
    }
    if keyboard.pressed(KeyCode::ArrowUp) {
        for mut velocity in query.iter_mut() {
            velocity.velocity = Vec2::new(0.0, 1.0);
        }
    }
    if keyboard.pressed(KeyCode::ArrowDown) {
        for mut velocity in query.iter_mut() {
            velocity.velocity = Vec2::new(0.0, -1.0);
        }
    }
    if keyboard.pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }
}

fn move_links(
    mut query: Query<(&mut Transform, Entity, &SnakeLink, &Velocity), With<SnakeLink>>,
    mut timer: ResMut<MovementTimer>,
    time: Res<Time>,
) {
    if !timer.timer.tick(time.delta()).just_finished() {
        return;
    }

    let mut m = HashMap::new();
    let mut tail = None;
    for (transform, entity, link, _) in query.iter_mut() {
        if let Some(previous_entity) = link.previous {
            m.insert(previous_entity, (entity, transform));
        } else {
            tail = Some((entity, transform));
        }
    }

    let Some(tail) = tail else {
        panic!("tail not found");
    };

    let mut cur = tail;
    loop {
        let next = m.get_mut(&cur.0);

        let Some(next) = next else {
            break;
        };

        cur.1.translation = next.1.translation;

        swap(&mut cur, next);
    }
    drop(m);

    for (mut transform, _, _, velocity) in query.iter_mut() {
        transform.translation += Vec3::new(velocity.velocity.x, 0.0, velocity.velocity.y);
    }
}

fn grow_links(
    mut events: EventReader<TreatEatenEvent>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut SnakeLink), With<SnakeTail>>,
) {
    for event in events.read() {
        let mut tail = query.single_mut();
        tail.1.previous = Some(event.treat_entity);
        commands.entity(event.treat_entity).insert((
            SnakeTail {},
            SnakeLink { previous: None },
            Velocity {
                velocity: Vec2::ZERO,
            },
        ));
        commands.entity(tail.0).remove::<SnakeTail>();
    }
}
