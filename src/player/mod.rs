use bevy::{color::palettes::css::BLUE, prelude::*};
use crate::camera::PlayerCameraTarget;
use controller::PlayerControllerPlugin;

mod controller;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
	fn build(&self, app: &mut App) {
		app
		.add_systems(Startup, spawn_player)
		.add_plugins(PlayerControllerPlugin);
	}
}

#[derive(Component)]
pub struct Player {
	pub speed: f32,
	pub sprint_speed: f32,
	pub sprint_enabled: bool,
}

impl Default for Player {
	fn default() -> Self {
		Player {
			speed: 1.0,
			sprint_speed: 2.0,
			sprint_enabled: true,
		}
	}
}

/// Player spawn system
fn spawn_player(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
	let player = (
		Mesh3d(meshes.add(Mesh::from(Cuboid::new(1.0, 1.0, 1.0)))),
        MeshMaterial3d(materials.add(Color::from(BLUE))),
		Transform::from_xyz(0.0, 0.5, 0.0),
		Player::default(), 
		controller::PlayerController::default(),
		PlayerCameraTarget
	);

	commands.spawn(player);
}