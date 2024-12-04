use super::{mouse::orbit_mouse, *,};
use bevy::window::{CursorGrabMode, PrimaryWindow};
use super::offset::{toggle_x_offset, toggle_x_offset_condition};
use {PlayerCameraTarget, PlayerCamera};

pub struct SharedPlugin;

impl Plugin for SharedPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(
			Update,
			(
				aim.run_if(aim_condition),
				sync_player_camera.after(orbit_mouse),
				toggle_x_offset.run_if(toggle_x_offset_condition),
				toggle_cursor.run_if(toggle_cursor_condition),
			),
		);
	}
}

fn sync_player_camera(
	player_q: Query<&Transform, With<PlayerCameraTarget>>,
	mut cam_q: Query<(&mut PlayerCamera, &mut Transform), Without<PlayerCameraTarget>>
) {
	let Ok(player) = player_q.get_single() else {
		return;
	};
	let Ok((cam, mut cam_transform)) = cam_q.get_single_mut() else {
		return;
	};

	// Calculate the desired camera translation based, radius, and xy_offset
	let rotation_matrix = Mat3::from_quat(cam_transform.rotation);

	// apply the offset if offset_enabled is true
	let mut offset = Vec3::ZERO;
	if cam.offset_enabled {
		offset = rotation_matrix.mul_vec3(Vec3::new(cam.offset.offset.0, cam.offset.offset.1, 0.0));
	}

	let desired_translation =
		rotation_matrix.mul_vec3(Vec3::new(0.0, 0.0, cam.zoom.radius)) + offset;

	// Update the camera translation
	cam_transform.translation = desired_translation + player.translation;
}

fn aim(
	mut cam_q: Query<(&mut PlayerCamera, &Transform), (With<PlayerCamera>, Without<PlayerCameraTarget>)>,
	mouse: Res<ButtonInput<MouseButton>>,
	mut player_q: Query<&mut Transform, With<PlayerCameraTarget>>, 
	time: Res<Time>
) {
	let Ok((mut cam, cam_transform)) = cam_q.get_single_mut() else {
		return;
	};

	// check if aim button was pressed
	let aim_btn = mouse.pressed(cam.aim_button);

	if aim_btn {
		// rotate player or target to face direction he is aiming
		let Ok(mut player_transform) = player_q.get_single_mut() else {
			return;
		};
		player_transform.look_to(*cam_transform.forward(), Vec3::Y);

		let desired_zoom = cam.zoom.min * cam.aim_zoom;

		// radius_copy is used for restoring the radius (zoom) to it's
		// original value after releasing the aim button
		if cam.zoom.radius_copy.is_none() {
			cam.zoom.radius_copy = Some(cam.zoom.radius);
		}

		let zoom_factor = (cam.zoom.radius_copy.unwrap() / cam.aim_zoom) * cam.aim_speed * time.delta_secs();

		// stop zooming in if current radius is less than desired zoom
		if cam.zoom.radius <= desired_zoom || cam.zoom.radius - zoom_factor <= desired_zoom {
			cam.zoom.radius = desired_zoom;
		} else {
			cam.zoom.radius -= zoom_factor;
		}
	} else {
		if let Some(radius_copy) = cam.zoom.radius_copy {
			let zoom_factor = (radius_copy / cam.aim_zoom) * cam.aim_speed * time.delta_secs();

			// stop zooming out if current radius is greater than original radius
			if cam.zoom.radius >= radius_copy || cam.zoom.radius + zoom_factor >= radius_copy {
				cam.zoom.radius = radius_copy;
				cam.zoom.radius_copy = None;
			} else {
				cam.zoom.radius +=
					(radius_copy / cam.aim_zoom) * cam.aim_speed * time.delta_secs();
			}
		}
	}
}

fn toggle_cursor(mut cam_q: Query<&mut PlayerCamera>, keys: Res<ButtonInput<KeyCode>>, mut window_q: Query<&mut Window, With<PrimaryWindow>>) {
	let Ok(mut cam) = cam_q.get_single_mut() else {
		return;
	};

	if keys.just_pressed(cam.cursor_lock_key) {
		cam.cursor_lock_active = !cam.cursor_lock_active;
	}

	let mut window = window_q.get_single_mut().unwrap();
	if cam.cursor_lock_active {
		window.cursor_options.grab_mode = CursorGrabMode::Locked;
		window.cursor_options.visible = false;
	} else {
		window.cursor_options.grab_mode = CursorGrabMode::None;
		window.cursor_options.visible = true;
	}
}

// checks if the toggle cursor functionality is enabled
fn toggle_cursor_condition(cam_q: Query<&PlayerCamera>) -> bool {
	let Ok(cam) = cam_q.get_single() else {
		return true;
	};
	cam.cursor_lock_toggle_enabled
}

// only run aiming logic if `aim_enabled` is true
fn aim_condition(cam_q: Query<&PlayerCamera, With<PlayerCamera>>) -> bool {
	let Ok(cam) = cam_q.get_single() else {
		return false;
	};
	cam.aim_enabled
}
