use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, spawn_player).add_systems(Update, player_movement);
	}
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Speed(f32);

/// Player movement system
fn player_movement(keys: Res<ButtonInput<KeyCode>>, time: Res<Time>, mut player: Query<(&mut Transform, &Speed), With<Player>>, camera: Query<&Transform, (With<Camera3d>, Without<Player>)>) {
	for (mut player_transform, player_speed) in player.iter_mut() {
		let cam: &Transform = match camera.get_single() {
			Ok(camera) => camera,
			Err(e) => Err(format!("Error retrieving camera: {}", e)).unwrap(),
		};

		let mut direction = Vec3::ZERO;

		if keys.pressed(KeyCode::KeyW) {
			direction += cam.forward().normalize_or_zero();
		}

		if keys.pressed(KeyCode::KeyS) {
			direction += cam.back().normalize_or_zero();
		}

		if keys.pressed(KeyCode::KeyA) {
			direction += cam.left().normalize_or_zero();
		}

		if keys.pressed(KeyCode::KeyD) {
			direction += cam.right().normalize_or_zero();
		}

		if keys.pressed(KeyCode::Space) {
			direction += Vec3::Y;
		}

		if keys.pressed(KeyCode::ControlLeft) {
			direction -= Vec3::Y;
		}

		if keys.pressed(KeyCode::Escape) {
			std::process::exit(0);
		}

		direction.y = 0.0;

		let movement = direction * player_speed.0 * time.delta_seconds();
		player_transform.translation += movement;
	}
}

/// Player spawn system
fn spawn_player(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
	let player = (PbrBundle {
		mesh: meshes.add(Mesh::from(Cuboid::new(1.0, 1.0, 1.0))),
		material: materials.add(Color::BLUE),
		transform: Transform::from_xyz(0.0, 0.5, 0.0),
		..default()
	}, Player, Speed(2.0));

	commands.spawn(player);
}