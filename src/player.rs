use crate::common::Position;
use crate::level::TreatEatenEvent;
use bevy::color::palettes::css::WHITE;
use bevy::prelude::*;
use bevy::utils::HashMap;
use std::mem::swap;
use std::time::Duration;

const DEFAULT_MOVEMENT_INTERVAL: Duration = Duration::from_millis(250);
const DEFAULT_LERP_RATE: f32 = 5.0;
const DIR_LEFT: IVec2 = IVec2::new(1, 0);
const DIR_RIGHT: IVec2 = IVec2::new(-1, 0);
const DIR_UP: IVec2 = IVec2::new(0, 1);
const DIR_DOWN: IVec2 = IVec2::new(0, -1);

#[derive(Resource)]
pub struct LerpRate {
    pub rate: f32,
}

#[derive(Component)]
pub struct SnakeHead;

#[derive(Component)]
pub struct SnakeTail;

#[derive(Component)]
pub struct SnakeLink {
    previous: Option<Entity>,
}

#[derive(Component)]
struct Direction {
    direction: IVec2,
}

#[derive(Resource)]
pub struct MovementTimer {
    pub timer: Timer,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LerpRate {
            rate: DEFAULT_LERP_RATE,
        });
        app.insert_resource(MovementTimer {
            timer: Timer::new(DEFAULT_MOVEMENT_INTERVAL, TimerMode::Repeating),
        });
        app.add_systems(Update, input);
        app.add_systems(Update, (move_links, grow_links).chain());
        app.add_systems(Startup, setup);
        app.add_systems(Update, interpolate_links);
    }
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands
        .spawn((
            SnakeLink { previous: None },
            SnakeTail,
            SnakeHead,
            PbrBundle {
                mesh: meshes.add(Mesh::from(Sphere::default())),
                material: materials.add(StandardMaterial {
                    base_color: Color::srgb(6.5, 6.5, 6.5),
                    unlit: true,
                    ..Default::default()
                }),
                ..Default::default()
            },
            Position {
                position: IVec2::ZERO,
                prev_position: IVec2::ZERO,
            },
            Direction {
                direction: IVec2::ZERO,
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
    mut query: Query<(&mut Direction, &Position), With<SnakeHead>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut exit: EventWriter<AppExit>,
) {
    if keyboard.pressed(KeyCode::ArrowLeft) {
        for (mut direction, position) in query.iter_mut() {
            if position.position - position.prev_position != DIR_RIGHT {
                direction.direction = DIR_LEFT;
            }
        }
    }
    if keyboard.pressed(KeyCode::ArrowRight) {
        for (mut direction, position) in query.iter_mut() {
            if position.position - position.prev_position != DIR_LEFT {
                direction.direction = DIR_RIGHT;
            }
        }
    }
    if keyboard.pressed(KeyCode::ArrowUp) {
        for (mut direction, position) in query.iter_mut() {
            if position.position - position.prev_position != DIR_DOWN {
                direction.direction = DIR_UP;
            }
        }
    }
    if keyboard.pressed(KeyCode::ArrowDown) {
        for (mut direction, position) in query.iter_mut() {
            if position.position - position.prev_position != DIR_UP {
                direction.direction = DIR_DOWN;
            }
        }
    }
    if keyboard.pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }
}

fn move_links(
    mut query: Query<(&mut Position, Entity, &SnakeLink, &Direction), With<SnakeLink>>,
    mut timer: ResMut<MovementTimer>,
    time: Res<Time>,
) {
    if !timer.timer.tick(time.delta()).just_finished() {
        return;
    }

    let mut m = HashMap::new();
    let mut tail = None;
    for (position, entity, link, _) in query.iter_mut() {
        if let Some(previous_entity) = link.previous {
            m.insert(previous_entity, (entity, position));
        } else {
            tail = Some((entity, position));
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

        cur.1.prev_position = cur.1.position;
        cur.1.position = next.1.position;

        swap(&mut cur, next);
    }
    drop(m);

    for (mut position, _, _, direction) in query.iter_mut() {
        if direction.direction.x != 0 || direction.direction.y != 0 {
            position.prev_position = position.position;
            position.position += direction.direction;
        }
    }
}

fn grow_links(
    mut events: EventReader<TreatEatenEvent>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut SnakeLink, &Position), With<SnakeTail>>,
) {
    for event in events.read() {
        let mut tail = query.single_mut();
        tail.1.previous = Some(event.treat_entity);
        commands.entity(event.treat_entity).insert((
            SnakeTail {},
            SnakeLink { previous: None },
            Position {
                position: tail.2.prev_position,
                prev_position: tail.2.prev_position,
            },
            Direction {
                direction: IVec2::ZERO,
            },
        ));
        commands.entity(tail.0).remove::<SnakeTail>();
    }
}

fn interpolate_links(
    mut query: Query<(&Position, &mut Transform), With<SnakeLink>>,
    fixed_time: Res<Time>,
    lerp_rate: Res<LerpRate>,
) {
    for (state, mut xf) in query.iter_mut() {
        let direction = Vec2::new(
            state.position.x as f32 - xf.translation.x,
            state.position.y as f32 - xf.translation.z,
        );
        xf.translation.x += direction.x * lerp_rate.rate * fixed_time.delta_seconds();
        xf.translation.z += direction.y * lerp_rate.rate * fixed_time.delta_seconds();
    }
}
