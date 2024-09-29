use crate::player::SnakeLink;
use bevy::color::palettes::css::{GREEN, GREY, RED, WHITE};
use bevy::prelude::*;

#[derive(Component)]
pub struct SnakeTreat;

pub struct LevelPlugin;

#[derive(Event)]
pub struct TreatEatenEvent {
    pub treat_entity: Entity,
}

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_event::<TreatEatenEvent>();
        app.add_systems(Update, collisions);
        app.add_systems(Update, spawn_treats);
    }
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(Cuboid::default())),
        material: materials.add(StandardMaterial {
            base_color: WHITE.into(),
            ..Default::default()
        }),
        transform: Transform::from_xyz(-4.0, 0.0, -4.0),
        ..Default::default()
    });

    // Sphere
    commands
        .spawn((
            SnakeTreat,
            PbrBundle {
                mesh: meshes.add(Mesh::from(Sphere::default())),
                material: materials.add(StandardMaterial {
                    base_color: Color::srgb(10.0, 0.0, 0.0),
                    unlit: true,
                    ..Default::default()
                }),
                transform: Transform::from_xyz(-2.0, 0.0, 0.0),
                ..Default::default()
            },
        ))
        .with_children(|children| {
            children.spawn(PointLightBundle {
                point_light: PointLight {
                    shadows_enabled: true,
                    intensity: 10_000_000.,
                    range: 100.0,
                    shadow_depth_bias: 0.1,
                    radius: 0.5,
                    color: RED.into(),
                    ..Default::default()
                },
                ..Default::default()
            });
        });

    // Sphere
    commands
        .spawn((
            SnakeTreat,
            PbrBundle {
                mesh: meshes.add(Mesh::from(Sphere::default())),
                material: materials.add(StandardMaterial {
                    base_color: Color::srgb(0.0, 10.0, 0.0),
                    unlit: true,
                    ..Default::default()
                }),
                transform: Transform::from_xyz(8.0, 0.0, 4.0),
                ..Default::default()
            },
        ))
        .with_children(|children| {
            children.spawn(PointLightBundle {
                point_light: PointLight {
                    shadows_enabled: true,
                    intensity: 10_000_000.,
                    range: 100.0,
                    shadow_depth_bias: 0.1,
                    radius: 0.5,
                    color: GREEN.into(),
                    ..Default::default()
                },
                ..Default::default()
            });
        });

    // span a plane behind the sphere
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(Plane3d {
            half_size: Vec2::new(10.0, 10.0),
            normal: Dir3::Y,
        })),
        material: materials.add(StandardMaterial {
            base_color: GREY.into(),
            ..Default::default()
        }),
        transform: Transform::from_xyz(0.0, -0.5, 0.0),
        ..Default::default()
    });
}

fn collisions(
    mut commands: Commands,
    mut events: EventWriter<TreatEatenEvent>,
    snake_treats: Query<(Entity, &Transform, &SnakeTreat)>,
    snake_links: Query<&Transform, With<SnakeLink>>,
) {
    // detect collisions between snake and treats, remove SnakeTreat component and send event
    for (treat_entity, treat_transform, _) in snake_treats.iter() {
        for link_transform in snake_links.iter() {
            if treat_transform
                .translation
                .distance(link_transform.translation)
                < 1.0
            {
                commands.entity(treat_entity).despawn_descendants();
                commands.entity(treat_entity).remove::<SnakeTreat>();
                events.send(TreatEatenEvent { treat_entity });
            }
        }
    }
}

fn spawn_treats(
    query: Query<&SnakeTreat>,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    if query.is_empty() {
        commands
            .spawn((
                SnakeTreat,
                PbrBundle {
                    mesh: meshes.add(Mesh::from(Sphere::default())),
                    material: materials.add(StandardMaterial {
                        base_color: Color::srgb(10.0, 0.0, 0.0),
                        unlit: true,
                        ..Default::default()
                    }),
                    transform: Transform::from_xyz(
                        (rand::random::<f32>() - 0.5) * 20.0,
                        0.0,
                        (rand::random::<f32>() - 0.5) * 20.0,
                    ),
                    ..Default::default()
                },
            ))
            .with_children(|children| {
                children.spawn(PointLightBundle {
                    point_light: PointLight {
                        shadows_enabled: true,
                        intensity: 10_000_000.,
                        range: 100.0,
                        shadow_depth_bias: 0.1,
                        radius: 0.5,
                        color: RED.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                });
            });
    }
}
