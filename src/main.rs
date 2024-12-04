use bevy::prelude::*;

mod camera;
mod light;
mod player;
mod utils;
mod level;
mod window;

fn main() {
	App::new().add_plugins((
		DefaultPlugins, 
		player::PlayerPlugin, 
		camera::Camera3DPlugin, 
		camera::PlayerCameraPlugin, 
		light::LightPlugin, 
		level::LevelPlugin, 
		utils::os_diags::ScreenDiagsTextPlugin,
		window::WindowSettingsPlugin,
	)).run();
}