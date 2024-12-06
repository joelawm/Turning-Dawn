use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, init_level);
	}
}

fn init_level(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
	let level_material = materials.add(StandardMaterial {
		base_color: Color::WHITE,
		..default()
	});

	// Ground
	commands.spawn((
        Collider::cuboid(1000., 0., 1000.),
		Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(1000.)))),
		MeshMaterial3d(level_material.clone()),
		Transform::IDENTITY,
    ));

	// Target Wall
    commands.spawn((
        Collider::cuboid(30., 30., 30.),
		Mesh3d(meshes.add(Cuboid::from_length(60.))),
		MeshMaterial3d(level_material.clone()),
		Transform::from_xyz(0., 0., -100.),
    ));
}