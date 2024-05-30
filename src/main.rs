use bevy::prelude::*;

mod camera;
mod light;
mod player;
mod utils;
mod world;

fn main() {
	App::new().add_plugins((DefaultPlugins, player::PlayerPlugin, camera::CameraPlugin, light::LightPlugin, world::WorldPlugin, utils::fps_counter::ScreenDiagsTextPlugin)).run();
}