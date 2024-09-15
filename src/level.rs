use bevy::color::palettes::css::{GREEN, GREY, RED, WHITE};
use bevy::prelude::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
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
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(Sphere::default())),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb(10.0, 0.0, 0.0),
                unlit: true,
                ..Default::default()
            }),
            ..Default::default()
        })
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
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(Sphere::default())),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb(0.0, 10.0, 0.0),
                unlit: true,
                ..Default::default()
            }),
            transform: Transform::from_xyz(8.0, 0.0, 4.0),
            ..Default::default()
        })
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