use bevy::{log, prelude::*};
use avian3d::prelude::*;

pub mod controller;
mod camera;
mod zoom;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
	fn build(&self, app: &mut App) {
		app
		.init_resource::<controller::PlayerControllerState>()
		.add_plugins(controller::PlayerControllerPlugin)
		.add_systems(Startup, spawn_player)
		.add_systems(Update, (camera::update_camera_controller, zoom::zoom))
		.add_systems(FixedUpdate, controller::update_movement);
	}
}

#[derive(Component)]
pub struct Player {
	pub velocity : Vec3,
    pub gravity : f32,
	pub speed: f32,
	pub forward_sprint_speed: f32,
	pub backward_sprint_speed: f32,
	pub sprint_enabled: bool,
}

impl Default for Player {
	fn default() -> Self {
		Player {
			velocity: Vec3::ZERO,
			gravity: 9.8,
			speed: 20.0,
			forward_sprint_speed: 2.0,
			backward_sprint_speed: 1.1,
			sprint_enabled: true,
		}
	}
}

/// Player spawn system
fn spawn_player(mut commands: Commands, meshes: Res<Assets<Mesh>>, asset_server: Res<AssetServer>) {
	let handle: Handle<Scene> = asset_server.load("models/Base_Character.glb#Scene0");

	let camera = commands.spawn((
		Camera3d::default(),
		Transform::IDENTITY,
		Projection::Perspective(PerspectiveProjection {
			fov: 103.0_f32.to_radians(),
			..default()
		}),
        camera::CameraController::default(),
	)).id();

	let player_entity = commands.spawn((
		SceneRoot(handle.clone()),
		Transform::IDENTITY,
		ColliderConstructorHierarchy::new(ColliderConstructor::ConvexDecompositionFromMesh),
		RigidBody::Kinematic,
	)).id();

	let player = commands.spawn((
		Transform::from_xyz(0.0, 0.0, 0.0),
		Player::default(), 
		controller::PlayerController::default(),
		Visibility::Visible,
	)).id();

	commands.entity(camera);
    commands.entity(player).add_child(player_entity).add_child(camera);
}
//0.0015340410077627612 4096, 2074