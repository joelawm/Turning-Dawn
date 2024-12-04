use bevy::prelude::*;
use super::Player;
use crate::camera::PlayerCamera;

pub struct PlayerControllerPlugin;

impl Plugin for PlayerControllerPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Update, (player_movement, update_camera_perspective));
	}
}

#[derive(Component)]
pub struct PlayerController {
	pub move_left: KeyCode,
	pub move_right: KeyCode,
	pub move_forward: KeyCode,
	pub move_back: KeyCode,
	pub sprint: KeyCode,
	pub jump: KeyCode,
	pub main_menu: KeyCode,
}

impl Default for PlayerController {
	fn default() -> Self {
		PlayerController {
			move_left: KeyCode::KeyA,
			move_right: KeyCode::KeyD,
			move_forward: KeyCode::KeyW,
			move_back: KeyCode::KeyS,
			sprint: KeyCode::ShiftLeft,
			jump: KeyCode::Space,
			main_menu: KeyCode::Escape,
		}
	}
}

/// Player movement system
fn player_movement(keys: Res<ButtonInput<KeyCode>>, time: Res<Time>, mut player: Query<(&mut Transform, &PlayerController, &Player), With<Player>>, camera: Query<&Transform, (With<Camera3d>, Without<Player>)>) {
	for (mut player_transform, controller, player) in player.iter_mut() {
		let cam: &Transform = match camera.get_single() {
			Ok(camera) => camera,
			Err(e) => Err(format!("Error retrieving camera: {}", e)).unwrap(),
		};

		let mut direction = Vec3::ZERO;

		// Move forward
		if keys.pressed(controller.move_forward) {
			direction += cam.forward().normalize_or_zero();
		}

		// Move back
		if keys.pressed(controller.move_back) {
			direction += cam.back().normalize_or_zero();
		}

		// Move left
		if keys.pressed(controller.move_left) {
			direction += cam.left().normalize_or_zero();
		}

		// Move right
		if keys.pressed(controller.move_right) {
			direction += cam.right().normalize_or_zero();
		}

		// Jump
		if keys.pressed(controller.jump) {
			direction += Vec3::Y;
		}

		// Main Menu
		if keys.pressed(controller.main_menu) {
			std::process::exit(0);
		}

		// sprint
		let mut sprint = 1.0;
		if keys.pressed(controller.sprint) && player.sprint_enabled {
			sprint = player.sprint_speed;
		}

		// Temp REMOVE after gravity is added
		direction.y = 0.0;

		let movement = direction * player.speed * sprint * time.delta_secs();
		player_transform.translation += movement;

		// rotate player to face direction he is currently moving
		if direction.length_squared() > 0.0 {
			player_transform.look_to(direction, Vec3::Y);
		}
	}
}

fn update_camera_perspective(keys: Res<ButtonInput<KeyCode>>, mut cam_q: Query<(&mut PlayerCamera, &mut Transform)>) {
	if keys.just_pressed(KeyCode::ArrowUp) {
		for (mut cam, mut transform) in cam_q.iter_mut() {
			if cam.is_first_person {
				cam.is_first_person = false;
				transform.translation = Vec3::new(-2.0, 2.5, 5.0);
			} else {
				cam.is_first_person = true;
				transform.translation = Vec3::new(-1.0, 5.5, 3.0);
			}
		};
	}
}
