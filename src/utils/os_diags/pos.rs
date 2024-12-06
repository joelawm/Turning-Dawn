use std::fmt::Write;
use bevy::{log, prelude::*};
use crate::player::controller::PlayerControllerState;
use super::DiagsState;

const X_MISSING: &str = "X: ???";
const Y_MISSING: &str = "Y: ???";
const Z_MISSING: &str = "Z: ???";
pub const POSITION_INITIAL: &str = "X: ... Y: ... Z: ...";

#[derive(Component)]
pub struct DiagsPos;

pub fn update(time: Res<Time<Real>>, player: Res<PlayerControllerState>, state_resource: Option<ResMut<DiagsState>>, mut text_query: Query<&mut Text, With<DiagsPos>>) {
	let mut state = match state_resource {
		None => {
			bevy::log::error!("DiagsPos state resource not found"); 
			return;
		},
		Some(state) => state,
	};

	if state.update_now || state.pos_timer.tick(time.delta()).just_finished() {
		if state.pos_timer.paused() {
			// Time is paused so remove text
			for mut text in text_query.iter_mut() {
				let value = &mut text.0;
				value.clear();
			}
		} else {
			for mut text in text_query.iter_mut() {
				//let borrowed_text = &mut text;
				let pos_value = &mut text.0;
				pos_value.clear();

				if let Err(e) = write!(pos_value, "X: {:.2} Y: {:.2} Z: {:.2}", player.position.x, player.position.y, player.position.z) {
					log::error!("Error writing position to text: {}", e);
					pos_value.clear();
					write!(pos_value, "X: {} Y: {} Z: {}", X_MISSING, Y_MISSING, Z_MISSING).unwrap();
				}
			}
		}
	}
}