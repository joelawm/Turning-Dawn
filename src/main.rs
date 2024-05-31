use bevy::prelude::*;

mod camera;
mod controller;
mod light;
mod player;
mod utils;
mod world;

fn main() {
	App::new().add_plugins((DefaultPlugins, player::PlayerPlugin, camera::Camera3DPlugin, camera::ThirdPersonCameraPlugin, light::LightPlugin, world::WorldPlugin, utils::os_diags::ScreenDiagsTextPlugin)).run();
}