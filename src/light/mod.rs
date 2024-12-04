use bevy::prelude::*;

pub struct LightPlugin;

impl Plugin for LightPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, spawn_light);
	}
}

fn spawn_light(mut commands: Commands) {
	let light = (
		DirectionalLight{illuminance: light_consts::lux::OVERCAST_DAY, shadows_enabled: true, ..default()},
        Transform::from_xyz(100., 200., 100.).looking_at(Vec3::ZERO, Vec3::Y),
    );	

	commands.spawn(light);
}