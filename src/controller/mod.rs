mod gamepad;
mod keyboard;

use bevy::prelude::*;
use gamepad::GamepadPlugin;
use keyboard::KeyboardPlugin;

pub struct ThirdPersonControllerPlugin;

impl Plugin for ThirdPersonControllerPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins((KeyboardPlugin, GamepadPlugin));
	}
}

#[derive(Component)]
pub struct ThirdPersonController {
	pub move_left: KeyCode,
	pub move_right: KeyCode,
	pub move_forward: KeyCode,
	pub move_back: KeyCode,
	pub sprint_enabled: bool,
	pub sprint: KeyCode,
	pub sprint_speed: f32,
	pub speed: f32,
	pub gamepad_settings: ControllerGamepadSettings,
}

impl Default for ThirdPersonController {
	fn default() -> Self {
		ThirdPersonController {
			move_left: KeyCode::KeyA,
			move_right: KeyCode::KeyD,
			move_forward: KeyCode::KeyW,
			move_back: KeyCode::KeyS,
			sprint_enabled: true,
			sprint: KeyCode::ShiftLeft,
			sprint_speed: 2.0,
			speed: 2.5,
			gamepad_settings: ControllerGamepadSettings::default(),
		}
	}
}

#[derive(Component)]
pub struct ControllerGamepadSettings {
	pub sprint: GamepadButton,
}

impl Default for ControllerGamepadSettings {
	fn default() -> Self {
		let gamepad = Gamepad::new(0);
		ControllerGamepadSettings {
			sprint: GamepadButton::new(gamepad, GamepadButtonType::LeftTrigger),
		}
	}
}
