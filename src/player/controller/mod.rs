use bevy::{log, prelude::*};
use bevy_rapier3d::prelude::*;

use super::Player;
use super::camera::CameraController;

pub struct PlayerControllerPlugin;

impl Plugin for PlayerControllerPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Update, (input_movement, update_camera_perspective, free_look));
	}
}

#[derive(Resource)]
pub struct PlayerControllerState {
	pub movement: Vec2,
	pub position: Vec3,
	pub sprint_speed: f32,
	pub direction: Vec2,
}

impl Default for PlayerControllerState {
	fn default() -> Self {
		Self {
			movement: Vec2::ZERO,
			position: Vec3::ZERO,
			sprint_speed: 0.0,
			direction: Vec2::ZERO,
		}
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
	pub camera_perspective: KeyCode,
	pub free_look: KeyCode,
	pub inventory: KeyCode,
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
			camera_perspective: KeyCode::ArrowUp,
			free_look: KeyCode::AltLeft,
			inventory: KeyCode::Tab,
		}
	}
}

/// Player movement system
pub fn input_movement(keys: Res<ButtonInput<KeyCode>>, mut player: Query<(&PlayerController, &mut Player, &Transform), With<Player>>, mut input: ResMut<PlayerControllerState>) {
	for (controller, player, transform) in player.iter_mut() {
		// Set the players position
		input.position = transform.translation;
		
		input.movement = Vec2::ZERO;
		input.sprint_speed = 1.0;

		// Move forward
		if keys.pressed(controller.move_forward) {
			if keys.pressed(controller.sprint) && player.sprint_enabled {
				input.sprint_speed = player.forward_sprint_speed;
			}

			input.movement.x += 1.;
		}

		// Move back
		if keys.pressed(controller.move_back) {
			if keys.pressed(controller.sprint) && player.sprint_enabled {
				input.sprint_speed = player.backward_sprint_speed;
			}

			input.movement.x -= 1.;
		}

		// Move left
		if keys.pressed(controller.move_left) {
			input.movement.y -= 1.;
		}

		// Move right
		if keys.pressed(controller.move_right) {
			input.movement.y += 1.;
		}

		// Jump
		if keys.pressed(controller.jump) {
			//input.movement.z += 1.;
		}

		// Main Menu
		if keys.pressed(controller.main_menu) {
			std::process::exit(0);
		}
	}
}

/// Updates the players movement
pub fn update_movement(
    time : Res<Time<Fixed>>,
    mut input : ResMut<PlayerControllerState>,
    camera_query : Query<&CameraController>,
    mut player_query : Query<(&mut Player, &mut Transform, &mut KinematicCharacterController, Option<&KinematicCharacterControllerOutput>)>,
){
    let Ok(camera) = camera_query.get_single() else {
		log::error!("Camera not found");
		return;
	};

    for(mut player, mut transform, mut controller, controller_output) in player_query.iter_mut() {
        if let Some(output) = controller_output{
            if output.grounded {
                player.velocity = Vec3::ZERO;
            }
        }

		// Get the camera current rotation in Radians
        let camera_rotation_converted = -camera.rotation.y.to_radians() - 90.0_f32.to_radians();

		// The forward direction of the camera
		// If free looking use the last looked direction else calculate the new direction
		let forward = if camera.is_free_looking {
			input.direction
		} else {
			// Calculate the forward direction of the camera
			let forward = Vec2::new(f32::cos(camera_rotation_converted), f32::sin(camera_rotation_converted));
			input.direction = forward;
			forward
		};

		// The right direction of the camera perpendicular to the forward direction
        let right = Vec2::new(-forward.y, forward.x);

		if let Some(movement_direction) = (forward * input.movement.x + right * input.movement.y).try_normalize() {
			player.velocity.x = movement_direction.x * player.speed * input.sprint_speed;
			player.velocity.z = movement_direction.y * player.speed * input.sprint_speed;
		}

		// Apply gravity
        player.velocity.y -= player.gravity * time.timestep().as_secs_f32();

        // Apply translation
        controller.translation = Some(player.velocity * time.timestep().as_secs_f32());

		// Rotate the player to face the direction of movement
		//transform.look_to(Vec3::new(forward.x, 0.0, forward.y), Vec3::Y);
    }
}

/// Update camera perspective system which allows a player to go between first and third person
fn update_camera_perspective(mut keys: ResMut<ButtonInput<KeyCode>>, player: Query<&PlayerController, With<Player>>, mut camera_query: Query<(&mut CameraController, &mut Transform)>) {
	let Some(controller) = player.iter().next() else {
		log::error!("Camera Perspective: PlayerController not found");
		return;
	};

	if keys.just_pressed(controller.camera_perspective) {
		for (mut cam, mut transform) in camera_query.iter_mut() {
			log::info!("Camera Perspective: {:?}", transform.translation);
			if cam.is_first_person {
				cam.is_first_person = false;
				transform.translation = Vec3::new(0.0, 5.5, 5.0);
				log::info!("Camera Perspective: {:?}", transform.translation);
			} else {
				cam.is_first_person = true;
				transform.translation = Vec3::new(0.0, 0.0, 0.0);
			}
		};
		keys.clear_just_pressed(controller.camera_perspective);
	}
}

/// Allows the user to free look as if they're turning their head
fn free_look(keys: Res<ButtonInput<KeyCode>>, player: Query<&PlayerController, With<Player>>, mut camera_query: Query<&mut CameraController>) {
	let Some(controller) = player.iter().next() else {
		log::error!("Free look: PlayerController not found");
		return;
	};

	if let Ok(mut camera_controller) = camera_query.get_single_mut() {
		if keys.pressed(controller.free_look) {
			camera_controller.is_free_looking = true;
		} else {
			camera_controller.is_free_looking = false;
		}
	}
}