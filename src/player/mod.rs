use bevy::{color::palettes::css::BLUE, prelude::*};
use bevy_rapier3d::prelude::*;

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
		.add_systems(Update, (camera::update_camera_controller, zoom::zoom_mouse))
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
fn spawn_player(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
	let camera = commands.spawn((
		Camera3d::default(),
		Transform::IDENTITY,
		Projection::Perspective(PerspectiveProjection {
			fov: 103.0_f32.to_radians(),
			..default()
		}),
        camera::CameraController::default(),
	)).id();

	let player = commands.spawn((
		Mesh3d(meshes.add(Mesh::from(Cuboid::new(1.0, 10.0, 1.0)))),
        MeshMaterial3d(materials.add(Color::from(BLUE))),
		Transform::from_xyz(0.0, 20., 0.0),
		Player::default(), 
		controller::PlayerController::default(),
		Collider::cuboid(1., 10., 1.),
        RigidBody::KinematicPositionBased,
        KinematicCharacterController{
            up : Vec3::Y,
            offset : CharacterLength::Absolute(0.01),
            ..default()
        }
	)).id();

	commands.entity(camera);
    commands.entity(player).add_child(camera);
}