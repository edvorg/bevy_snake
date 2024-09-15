use bevy::color::palettes::css::WHITE;
use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
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
        ))
        .with_children(|children| {
            children.spawn(PointLightBundle {
                point_light: PointLight {
                    shadows_enabled: true,
                    intensity: 10_000_000.,
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

fn movement(
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.pressed(KeyCode::ArrowLeft) {
        for mut transform in query.iter_mut() {
            transform.translation.x -= 3.0 * time.delta_seconds();
        }
    }
    if keyboard.pressed(KeyCode::ArrowRight) {
        for mut transform in query.iter_mut() {
            transform.translation.x += 3.0 * time.delta_seconds();
        }
    }
    if keyboard.pressed(KeyCode::ArrowUp) {
        for mut transform in query.iter_mut() {
            transform.translation.z -= 3.0 * time.delta_seconds();
        }
    }
    if keyboard.pressed(KeyCode::ArrowDown) {
        for mut transform in query.iter_mut() {
            transform.translation.z += 3.0 * time.delta_seconds();
        }
    }
}
