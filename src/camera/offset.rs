use bevy::prelude::*;
use super::PlayerCamera;

/// Offset the camera behind the player. For example, an offset value of (0.5, 0.25) will
/// place the camera closer the player's right shoulder
pub struct Offset {
	pub offset: (f32, f32),
	offset_copy: (f32, f32),
	is_transitioning: bool,
}

impl Offset {
	pub fn new(x: f32, y: f32) -> Self {
		Self {
			offset: (x, y),
			offset_copy: (x, y),
			is_transitioning: false,
		}
	}
}

// only run toggle_x_offset if `offset_toggle_enabled` is true
pub fn toggle_x_offset_condition(cam_q: Query<&PlayerCamera, With<PlayerCamera>>) -> bool {
	let Ok(cam) = cam_q.get_single() else {
		return false;
	};
	cam.offset_toggle_enabled
}

// inverts the x offset. Example: left shoulder view -> right shoulder view & vice versa
pub fn toggle_x_offset(mut cam_q: Query<&mut PlayerCamera, With<PlayerCamera>>, 
	keys: Res<ButtonInput<KeyCode>>, time: Res<Time>) {
	let Ok(mut cam) = cam_q.get_single_mut() else {
		return;
	};

	// check if toggle btn was pressed
	let toggle_btn = keys.just_pressed(cam.offset_toggle_key);

	if toggle_btn {
		// Switch direction by inverting the offset_flag
		cam.offset.is_transitioning = !cam.offset.is_transitioning;
	}

	// Determine the transition speed based on direction
	let transition_speed = if cam.offset.is_transitioning {
		-cam.offset_toggle_speed
	} else {
		cam.offset_toggle_speed
	};

	// Update the offset based on the direction and time
	cam.offset.offset.0 = (cam.offset.offset.0 + transition_speed * time.delta_secs())
		.clamp(-cam.offset.offset_copy.0, cam.offset.offset_copy.0);
}