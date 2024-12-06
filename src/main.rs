use bevy::prelude::*;
use bevy_rapier3d::plugin::{NoUserData, RapierPhysicsPlugin};

mod light;
mod player;
mod utils;
mod level;
mod window;

fn main() {
	App::new().add_plugins((
		DefaultPlugins,
		RapierPhysicsPlugin::<NoUserData>::default(), 
		player::PlayerPlugin, 
		light::LightPlugin, 
		level::LevelPlugin, 
		utils::os_diags::DebugMenuPlugin,
		window::WindowSettingsPlugin,
	)).run();
}