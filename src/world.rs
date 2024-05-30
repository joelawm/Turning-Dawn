use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, spawn_floor);
	}
}

fn spawn_floor(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
	let floor =  PbrBundle {
		mesh: meshes.add(Plane3d::default().mesh().size(15.0, 15.0)),
		material: materials.add(Color::rgb(0.3, 0.5, 0.3)),
		..default()
	};

	commands.spawn(floor);
}