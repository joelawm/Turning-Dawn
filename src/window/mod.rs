use bevy::{prelude::*, window::{PrimaryWindow, WindowMode, WindowResolution}};

mod cursor;

pub struct WindowSettingsPlugin;

impl Plugin for WindowSettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(cursor::CursorPlugin)
            .add_systems(PreStartup, init_window);
    }
}

fn init_window(mut window_query: Query<&mut Window, With<PrimaryWindow>>) {
    if let Ok(mut window) = window_query.get_single_mut() {
        window.title = "Turning Dawn".to_string();
        window.resolution = WindowResolution::new(1920., 1080.);
        window.mode = WindowMode::Windowed;
        //window.resolution = WindowResolution::new(2560., 1440.);
        //window.mode = WindowMode::BorderlessFullscreen(MonitorSelection::Primary);
    }
}