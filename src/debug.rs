use crate::player::LerpRate;
use bevy::app::{App, Plugin, Update};
use bevy::prelude::ResMut;
use bevy_egui::egui::{DragValue, Widget, Window};
use bevy_egui::{EguiContexts, EguiPlugin};
use std::ops::RangeInclusive;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin);
        app.add_systems(Update, ui_example_system);
    }
}

fn ui_example_system(mut contexts: EguiContexts, mut lerp_rate: ResMut<LerpRate>) {
    Window::new("Debug").show(contexts.ctx_mut(), |ui| {
        DragValue::new(&mut lerp_rate.rate)
            .prefix("Lerp Rate: ")
            .max_decimals(2)
            .speed(0.1)
            .clamp_to_range(true)
            .range(RangeInclusive::new(0.0, 1000.0))
            .ui(ui);
    });
}
