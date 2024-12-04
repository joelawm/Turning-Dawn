use std::fmt::Write;
use bevy::prelude::*;
use crate::player::Player;
use super::ScreenDiagsState;

const X_MISSING: &str = "X: ???";
const Y_MISSING: &str = "Y: ???";
const Z_MISSING: &str = "Z: ???";
pub const POSITION_INITIAL: &str = "X: ... Y: ... Z: ...";

#[derive(Component)]
pub struct ScreenDiagsPos;

pub fn update(time: Res<Time>, player_query: Query<&Transform, With<Player>>, state_resource: Option<ResMut<ScreenDiagsState>>, mut text_query: Query<&mut Text, With<ScreenDiagsPos>>) {
	let mut state = match state_resource {
		None => {
			bevy::log::error!("ScreenDiagsPos state resource not found"); 
			return;
		},
		Some(state) => state,
	};

	if state.update_now || state.timer.tick(time.delta()).just_finished() {
		if state.timer.paused() {
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

				if let Some(pos) = player_query.iter().next() {
					match write!(pos_value, "X: {:.2} Y: {:.2} Z: {:.2}", pos.translation.x, pos.translation.y, pos.translation.z) {
						Ok(_) => {},
						Err(e) => {
							bevy::log::error!("Error writing position to text: {}", e);
							pos_value.clear();
							write!(pos_value, "{} {} {}", X_MISSING, Y_MISSING, Z_MISSING).unwrap();
						}
					}
				} else {
					pos_value.clear();
					write!(pos_value, "{} {} {}", X_MISSING, Y_MISSING, Z_MISSING).unwrap();
				}
			}
		}
	}
}