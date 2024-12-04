use bevy::{input::mouse::MouseMotion, prelude::*, window::PrimaryWindow};
use std::f32::consts::PI;
use zoom::{ zoom_condition, zoom_mouse};
use crate::camera::PlayerCamera;

pub mod zoom;

pub struct MousePlugin;

impl Plugin for MousePlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Update, (orbit_mouse.run_if(orbit_condition), zoom_mouse.run_if(zoom_condition)).chain());
	}
}

// heavily referenced https://bevy-cheatbook.github.io/cookbook/pan-orbit-camera.html
pub fn orbit_mouse(
	window_q: Query<&Window, With<PrimaryWindow>>,
	mut cam_q: Query<(&PlayerCamera, &mut Transform), With<PlayerCamera>>,
	mouse: Res<ButtonInput<MouseButton>>,
	mut mouse_evr: EventReader<MouseMotion>,
) {
	let mut rotation = Vec2::ZERO;
	for ev in mouse_evr.read() {
		rotation = ev.delta;
	}

	let Ok((cam, mut cam_transform)) = cam_q.get_single_mut() else {
		return;
	};

	if cam.mouse_orbit_button_enabled && !mouse.pressed(cam.mouse_orbit_button) {
		return;
	}

	if rotation.length_squared() > 0.0 {
		let window = window_q.get_single().unwrap();
		let delta_x = {
			let delta = rotation.x / window.width() * std::f32::consts::PI * cam.sensitivity.x;
			delta
		};

		let delta_y = rotation.y / window.height() * PI * cam.sensitivity.y;
		let yaw = Quat::from_rotation_y(-delta_x);
		let pitch = Quat::from_rotation_x(-delta_y);
		cam_transform.rotation = yaw * cam_transform.rotation; // rotate around global y axis

		// Calculate the new rotation without applying it to the camera yet
		let new_rotation = cam_transform.rotation * pitch;

		// check if new rotation will cause camera to go beyond the 180 degree vertical bounds
		let up_vector = new_rotation * Vec3::Y;
		if up_vector.y > 0.0 {
			cam_transform.rotation = new_rotation;
		}
	}

	let rot_matrix = Mat3::from_quat(cam_transform.rotation);
	cam_transform.translation = rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, cam.zoom.radius));
}

// only run the orbit system if the cursor lock is disabled
fn orbit_condition(cam_q: Query<&PlayerCamera>) -> bool {
	let Ok(cam) = cam_q.get_single() else {
		return true;
	};
	return cam.cursor_lock_active;
}
