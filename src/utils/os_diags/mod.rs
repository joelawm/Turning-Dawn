use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, utils::Duration};
//SystemInformationDiagnosticsPlugin, EntityCountDiagnosticsPlugin

mod fps;
mod pos;

const FONT_SIZE: f32 = 18.0;
const FONT_COLOR: Color = Color::WHITE;
const UPDATE_INTERVAL: Duration = Duration::from_secs(1);

/// A plugin that draws diagnostics on-screen with Bevy UI.
/// Currently only the FPS is displayed.
///
/// Use the [marker struct](ScreenDiagsText) to customize the FPS counter appearance,
/// and the [resource](ScreenDiagsState) to control its behaviour.
pub struct ScreenDiagsPlugin;

impl Plugin for ScreenDiagsPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins(FrameTimeDiagnosticsPlugin::default()).init_resource::<ScreenDiagsState>();
	}
}

/// A plugin to write the FPS counter to the screen
///
/// Use the [marker struct](ScreenDiagsText) to customize the FPS counter appearance,
/// and the [resource](ScreenDiagsState) to control its behaviour.
pub struct ScreenDiagsTextPlugin;

impl Plugin for ScreenDiagsTextPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins(ScreenDiagsPlugin).add_systems(Startup, spawn_text).add_systems(Update, fps::update);
	}
}

/// The diagnostics state resource.
///
/// To disable the FPS counter, get a [ResMut](bevy::prelude::ResMut) reference to this struct and
/// pause the timer. Unpause the timer to re-enable the counter.
#[derive(Resource)]
pub struct ScreenDiagsState {
	/// The timer that triggers a diagnostics reading.
	/// Public, to allow flexible use, but in general use the methods to interact.
	pub timer: Timer,
	/// A flag to indicate to update the display, even if the timer has not popped.
	/// Public, to allow flexible use, but in general use the methods to interact.
	pub update_now: bool,
}

impl Default for ScreenDiagsState {
	fn default() -> Self {
		Self { timer: Timer::new(UPDATE_INTERVAL, TimerMode::Repeating), update_now: true }
	}
}

impl ScreenDiagsState {
	/// Enable the FPS display.
	pub fn enable(&mut self) {
		self.timer.unpause();
		self.update_now = true;
	}

	/// Disable the FPS display.
	pub fn disable(&mut self) {
		self.timer.pause();
		self.update_now = true;
	}

	/// Is the FPS display enabled.
	pub fn enabled(&self) -> bool {
		!self.timer.paused()
	}
}

fn spawn_text(mut commands: Commands, asset_server: Res<AssetServer>) {
	let column_id = commands.spawn(NodeBundle {
		style: Style {
			justify_content: JustifyContent::FlexStart,
			flex_direction: FlexDirection::Column,
			width: Val::Percent(16.),
			height: Val::Percent(95.),
			overflow: Overflow::clip_x(),
			..Default::default()
		}, ..Default::default()}).id();

	let font_fps = asset_server.load("font/screen-diags-font.ttf");
	let font_pos = asset_server.load("font/screen-diags-font.ttf");

	let sections = vec![TextSection {value: fps::FPS_INITIAL.to_string(), style: TextStyle {font: font_fps, font_size: FONT_SIZE, color: FONT_COLOR}}];
	let position_section = vec![TextSection {value: pos::POSITION_INITIAL.to_string(), style: TextStyle {font: font_pos, font_size: FONT_SIZE, color: FONT_COLOR}}];

	let bundle = TextBundle {text: Text {sections, ..Default::default()}, ..Default::default()};
	let position_bundle = TextBundle {text: Text {sections: position_section, ..Default::default()}, ..Default::default()};

	let fps_id = commands.spawn(bundle).insert(fps::ScreenDiagsFPS).id();
	let pos_id = commands.spawn(position_bundle).insert(pos::ScreenDiagsPos).id();

	commands.entity(column_id).add_child(fps_id).add_child(pos_id);
}