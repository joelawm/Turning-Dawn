use bevy::{input::mouse::MouseMotion, prelude::*};
use super::zoom::Zoom;

#[derive(Component)]
pub struct CameraController {
	pub is_first_person: bool,
	pub rotation: Vec2,
    pub rotation_lock: f32,
    pub sensitivity: f32,
    pub zoom: Zoom,
    pub zoom_sensitivity: f32
}

impl Default for CameraController {
	fn default() -> Self {
		CameraController {
			is_first_person: false,
			rotation: Vec2::ZERO,
			rotation_lock: 88.0,
			sensitivity: 0.035,
            zoom: Zoom::new(1.5, 3.0),
			zoom_sensitivity: 1.0
		}
	}
}

/// Update the camera controller rotation
pub fn update_camera_controller(mut mouse_motion: EventReader<MouseMotion>, mut camera_query: Query<(&mut CameraController, &mut Transform)>) {
    if let Ok((mut camera_controller, mut transform)) = camera_query.get_single_mut() {
        for ev in mouse_motion.read() {
            camera_controller.rotation.y -= ev.delta.x * camera_controller.sensitivity;
            camera_controller.rotation.x -= ev.delta.y * camera_controller.sensitivity;

            camera_controller.rotation.x = f32::clamp(camera_controller.rotation.x, -camera_controller.rotation_lock, camera_controller.rotation_lock);
        }

        let y_quat = Quat::from_axis_angle(Vec3::Y, camera_controller.rotation.y.to_radians());
        let x_quat = Quat::from_axis_angle(Vec3::X, camera_controller.rotation.x.to_radians());
        transform.rotation = y_quat * x_quat;

        let rot_matrix = Mat3::from_quat(transform.rotation);
        transform.translation = rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, camera_controller.zoom.radius));
    }
}
