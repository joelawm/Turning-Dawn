use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::{prelude::*, transform};
use bevy::window::PrimaryWindow;
use rand::prelude::*;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0; // This is the player sprite size.
pub const NUMBER_OF_ENEMIES: usize = 4;

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_plugins(LogDiagnosticsPlugin::default())
		.add_plugins(FrameTimeDiagnosticsPlugin::default())
		.add_systems(Startup, spawn_camera)
		.add_systems(Startup, spawn_player)
		.add_systems(Startup, spawn_enemies)
		.add_systems(Update, player_movement)
		.add_systems(Update, confine_player_movement)
		.run();
}

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Enemy {}

pub fn spawn_player(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>, asset_server: Res<AssetServer>) {
	let window = window_query.get_single().unwrap();

	commands.spawn((
		SpriteBundle {
			transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
			texture: asset_server.load("sprites/ball_blue_large.png"),
			..default()
		},
		Player {},
	));
}

pub fn spawn_enemies(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>,asset_server: Res<AssetServer>) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_ENEMIES {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Enemy {},
        ));
    }
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
	let window = window_query.get_single().unwrap();

	commands.spawn(Camera2dBundle {
		transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 1.0),
		..default()
	});
}

pub fn player_movement(keyboard_input: Res<Input<KeyCode>>, mut player_query: Query<&mut Transform, With<Player>>, time: Res<Time>) {
	if let Ok(mut transform) = player_query.get_single_mut() {
		let mut direction = Vec3::ZERO;

		if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
			direction += Vec3::new(-1.0, 0.0, 0.0);
		}
		if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
			direction += Vec3::new(1.0, 0.0, 0.0);
		}
		if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
			direction += Vec3::new(0.0, 1.0, 0.0);
		}
		if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
			direction += Vec3::new(0.0, -1.0, 0.0);
		}

		if direction.length() > 0.0 {
			direction = direction.normalize();
		}

		transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
	}
}

pub fn confine_player_movement(mut player_query: Query<&mut Transform, With<Player>>, window_query: Query<&Window, With<PrimaryWindow>>) {
	if let Ok(mut player_transform) = player_query.get_single_mut() {
		let window = window_query.get_single().unwrap();

		let half_player_size = PLAYER_SIZE / 2.0; // 32.0
		let x_min = 0.0 + half_player_size;
		let x_max = window.width() - half_player_size;
		let y_min = 0.0 + half_player_size;
		let y_max = window.height() - half_player_size;

		let mut translation = player_transform.translation;

		// Bound the player x position
		if translation.x < x_min {
			translation.x = x_min;
		} else if translation.x > x_max {
			translation.x = x_max;
		}
		// Bound the players y position.
		if translation.y < y_min {
			translation.y = y_min;
		} else if translation.y > y_max {
			translation.y = y_max;
		}

		player_transform.translation = translation;
	}
}

/// set up a simple 3D scene
fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}