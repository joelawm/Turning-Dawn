use std::fmt::Write;
use bevy::{diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin}, log, prelude::*};
use super::DiagsState;

const FPS_FORMAT: &str = "FPS: ";
pub const FPS_INITIAL: &str = "FPS: ...\n";
const FPS_MISSING: &str = "FPS: ???";

/// The marker on the text to be updated.
#[derive(Component)]
pub struct DiagsFPS;

pub fn update(time: Res<Time<Real>>, diagnostics: Res<DiagnosticsStore>, state_resource: Option<ResMut<DiagsState>>, mut text_query: Query<&mut Text, With<DiagsFPS>>) {
	let mut state = match state_resource {
		None => {
			bevy::log::error!("DiagsFPS state resource not found"); 
			return;
		},
		Some(state) => state,
	};

	if state.update_now || state.fps_timer.tick(time.delta()).just_finished() {
		if state.fps_timer.paused() {
			// Time is paused so remove text
			for mut text in text_query.iter_mut() {
				let value = &mut text.0;
				value.clear();
			}
		} else {
			let fps_diags = extract_fps(&diagnostics);

			for mut text in text_query.iter_mut() {
				let fps_value = &mut text.0;
				fps_value.clear();

				if let Some(fps) = fps_diags {
					match write!(fps_value, "{}{:.0}", FPS_FORMAT, fps) {
						Ok(_) => {},
						Err(e) => {
							log::error!("Error writing FPS to text: {}", e);
							fps_value.clear();
							write!(fps_value, "{}", FPS_MISSING).unwrap();
						}
					}
				} else {
					fps_value.clear();
					write!(fps_value, "{}", FPS_MISSING).unwrap();
				}
			}
		}
	}
}

/// Get the current fps
pub fn extract_fps(diagnostics: &Res<DiagnosticsStore>) -> Option<f64> {
	diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS).and_then(|fps| fps.smoothed())
}