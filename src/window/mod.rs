use bevy::{prelude::*, window::{PrimaryWindow, WindowMode, WindowResolution}};

pub struct WindowSettingsPlugin;
impl Plugin for WindowSettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, init_window);
    }
}

fn init_window(mut window_query: Query<&mut Window, With<PrimaryWindow>>) {
    if let Ok(mut window) = window_query.get_single_mut() {
        window.resolution = WindowResolution::new(2560., 1440.);
        window.mode = WindowMode::BorderlessFullscreen(MonitorSelection::Primary);
    }
}