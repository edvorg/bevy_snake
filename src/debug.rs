use crate::player::{LerpRate, MovementTimer};
use bevy::app::{App, Plugin, Update};
use bevy::prelude::ResMut;
use bevy_egui::egui::{DragValue, Widget, Window};
use bevy_egui::{EguiContexts, EguiPlugin};
use std::ops::RangeInclusive;
use std::time::Duration;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin);
        app.add_systems(Update, ui_example_system);
    }
}

fn ui_example_system(
    mut contexts: EguiContexts,
    mut lerp_rate: ResMut<LerpRate>,
    mut timer: ResMut<MovementTimer>,
) {
    Window::new("Debug").show(contexts.ctx_mut(), |ui| {
        let mut duration = timer.timer.duration().as_millis() as u64;
        if DragValue::new(&mut duration)
            .prefix("Step duration: ")
            .max_decimals(0)
            .speed(1.0)
            .clamp_to_range(true)
            .range(RangeInclusive::new(0, 1000))
            .ui(ui)
            .changed()
        {
            timer.timer.set_duration(Duration::from_millis(duration));
        }
        DragValue::new(&mut lerp_rate.rate)
            .prefix("Lerp Rate: ")
            .max_decimals(2)
            .speed(0.1)
            .clamp_to_range(true)
            .range(RangeInclusive::new(0.0, 1000.0))
            .ui(ui);
    });
}
