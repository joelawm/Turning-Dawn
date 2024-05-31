use std::fmt::Write;
use bevy::{diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin}, prelude::*};
use super::ScreenDiagsState;

const FPS_FORMAT: &str = "FPS: ";
pub const FPS_INITIAL: &str = "FPS: ...\n";
const FPS_MISSING: &str = "FPS: ???";

/// The marker on the text to be updated.
#[derive(Component)]
pub struct ScreenDiagsFPS;

pub fn update(time: Res<Time>, diagnostics: Res<DiagnosticsStore>, state_resource: Option<ResMut<ScreenDiagsState>>, mut text_query: Query<&mut Text, With<ScreenDiagsFPS>>) {
	let mut state = match state_resource {
		None => return,
		Some(state) => state,
	};

	if state.update_now || state.timer.tick(time.delta()).just_finished() {
		if state.timer.paused() {
			// Time is paused so remove text
			for mut text in text_query.iter_mut() {
				let value = &mut text.sections[0].value;
				value.clear();
			}
		} else {
			let fps_diags = extract_fps(&diagnostics);

			for mut text in text_query.iter_mut() {
				//let borrowed_text = &mut text;
				let fps_value = &mut text.sections[0].value;
				fps_value.clear();

				if let Some(fps) = fps_diags {
					write!(fps_value, "{}{:.0}", FPS_FORMAT, fps).unwrap();
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