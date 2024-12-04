use bevy::{input::mouse::MouseWheel, prelude::*};
use super::PlayerCamera;

/// Sets the zoom bounds (min & max)
pub struct Zoom {
	pub min: f32,
	pub max: f32,
	pub radius: f32,
	pub radius_copy: Option<f32>,
}

impl Zoom {
	pub fn new(min: f32, max: f32) -> Self {
		Self {
			min,
			max,
			radius: (min + max) / 2.0,
			radius_copy: None,
		}
	}
}

pub fn zoom_mouse(mut scroll_evr: EventReader<MouseWheel>, mut cam_q: Query<&mut PlayerCamera>) {
	let mut scroll = 0.0;
	for ev in scroll_evr.read() {
		scroll += ev.y;
	}

	if let Ok(mut cam) = cam_q.get_single_mut() {
		if scroll.abs() > 0.0 {
			let new_radius =
				cam.zoom.radius - scroll * cam.zoom.radius * 0.1 * cam.zoom_sensitivity;
			cam.zoom.radius = new_radius.clamp(cam.zoom.min, cam.zoom.max);
		}
	}
}

// only zoom if zoom is enabled & the cursor lock feature is enabled & active
pub fn zoom_condition(cam_q: Query<&PlayerCamera, With<PlayerCamera>>) -> bool {
	let Ok(cam) = cam_q.get_single() else {
		return false;
	};
	return cam.zoom_enabled && cam.cursor_lock_active;
}