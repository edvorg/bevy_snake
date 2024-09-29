mod camera;
mod common;
mod level;
mod player;

use crate::camera::CameraPlugin;
use crate::level::LevelPlugin;
use crate::player::PlayerPlugin;
use bevy::color::palettes::css::BLACK;
use bevy::prelude::*;
use bevy::render::settings::{Backends, RenderCreation, WgpuSettings};
use bevy::render::RenderPlugin;

fn main() {
    #[cfg(target_arch = "wasm32")]
    const BACKENDS: Backends = Backends::GL;
    #[cfg(not(target_arch = "wasm32"))]
    const BACKENDS: Backends = Backends::VULKAN;

    App::new()
        .insert_resource(ClearColor(BLACK.into()))
        .add_plugins(DefaultPlugins.set(RenderPlugin {
            render_creation: RenderCreation::Automatic(WgpuSettings {
                backends: Some(BACKENDS),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(PlayerPlugin)
        .add_plugins(LevelPlugin)
        .add_plugins(CameraPlugin)
        .run();
}
