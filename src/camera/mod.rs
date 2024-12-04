use bevy::prelude::*;
use mouse::MousePlugin;
use shared::SharedPlugin;
use crate::camera::offset::Offset;
use crate::camera::mouse::zoom::Zoom;

mod mouse;
mod offset;
mod shared;

#[derive(Component)]
pub struct PlayerCameraTarget;

#[derive(Component)]
pub struct PlayerCamera {
	pub is_first_person: bool,
	pub aim_enabled: bool,
	pub aim_button: MouseButton,
	pub aim_speed: f32,
	pub aim_zoom: f32,
	pub cursor_lock_toggle_enabled: bool,
	pub cursor_lock_active: bool,
	pub cursor_lock_key: KeyCode,
	pub sensitivity: Vec2,
	pub mouse_orbit_button_enabled: bool,
	pub mouse_orbit_button: MouseButton,
	pub offset_enabled: bool,
	pub offset: offset::Offset,
	pub offset_toggle_enabled: bool,
	pub offset_toggle_key: KeyCode,
	pub offset_toggle_speed: f32,
	pub zoom_enabled: bool,
	pub zoom: Zoom,
	pub zoom_sensitivity: f32,
}

impl Default for PlayerCamera {
	fn default() -> Self {
		PlayerCamera {
			is_first_person: false,
			aim_enabled: false,
			aim_button: MouseButton::Right,
			aim_speed: 3.0,
			aim_zoom: 0.7,
			cursor_lock_key: KeyCode::Escape,
			cursor_lock_toggle_enabled: true,
			cursor_lock_active: true,
			sensitivity: Vec2::new(1.0, 1.0),
			mouse_orbit_button_enabled: false,
			mouse_orbit_button: MouseButton::Middle,
			offset_enabled: false,
			offset: Offset::new(0.5, 0.4),
			offset_toggle_enabled: false,
			offset_toggle_speed: 5.0,
			offset_toggle_key: KeyCode::KeyE,
			zoom_enabled: true,
			zoom: Zoom::new(1.5, 3.0),
			zoom_sensitivity: 1.0,
		} 
	}
}

pub struct PlayerCameraPlugin;

impl Plugin for PlayerCameraPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins((MousePlugin, SharedPlugin));
	}
}

pub struct Camera3DPlugin;

impl Plugin for Camera3DPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, spawn_camera);
	}
}

fn spawn_camera(mut commands: Commands) {
	let camera = (
		Camera3d::default(),
		Projection::Perspective(PerspectiveProjection {
			fov: 103.0,
			..default()
		}),
		Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
		PlayerCamera::default()
	);

	commands.spawn(camera);
}