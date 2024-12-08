use bevy::{input::mouse::MouseWheel, prelude::*};
use super::camera::CameraController;

/// Sets the zoom bounds (min & max)
pub struct Zoom {
	pub min: f32,
	pub max: f32,
	pub radius: f32,

}

impl Zoom {
	pub fn new(min: f32, max: f32) -> Self {
		Self {min, max, radius: (min + max) / 2.0}
	}
}

pub fn zoom(mut scroll_evr: EventReader<MouseWheel>, mut cam_q: Query<&mut CameraController>) {
	let mut scroll = 0.0;
	for ev in scroll_evr.read() {
		scroll += ev.y;
	}

	if let Ok(mut cam) = cam_q.get_single_mut() {
		if cam.is_first_person {
			return;
		}
		
		if scroll.abs() > 0.0 {
			let new_radius = cam.zoom.radius - scroll * cam.zoom.radius * 0.1 * cam.zoom_sensitivity;
			cam.zoom.radius = new_radius.clamp(cam.zoom.min, cam.zoom.max);
		}
	}
}