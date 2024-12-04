use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, text::FontSmoothing, utils::Duration};

mod fps;
mod pos;

const FONT_SIZE: f32 = 18.0;
const FONT_COLOR: Color = Color::WHITE;
const UPDATE_INTERVAL: Duration = Duration::from_millis(100);

pub struct ScreenDiagsPlugin;

impl Plugin for ScreenDiagsPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins(FrameTimeDiagnosticsPlugin::default()).init_resource::<ScreenDiagsState>();
	}
}

pub struct ScreenDiagsTextPlugin;

impl Plugin for ScreenDiagsTextPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins(ScreenDiagsPlugin).add_systems(Startup, spawn_text).add_systems(Update, fps::update).add_systems(Update, pos::update);
	}
}

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
		Self { timer: Timer::new(UPDATE_INTERVAL, TimerMode::Repeating), update_now: false }
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
	let column_id = commands.spawn(Node {
		display: Display::Flex,	
		justify_content: JustifyContent::FlexStart,
		width: Val::Percent(16.),
		height: Val::Percent(95.),
		overflow: Overflow::clip_x(),
		flex_direction: FlexDirection::Column,
		..Default::default()
	}).id();

	let font = asset_server.load("font/screen-diags-font.ttf");

	let fps_section = (
		Text::new(fps::FPS_INITIAL.to_string()),
		TextFont {font: font.clone().into(), font_size: FONT_SIZE, font_smoothing: FontSmoothing::AntiAliased, ..Default::default()},
		TextColor(Color::from(FONT_COLOR))
	);

	let position_section = (
		Text::new(pos::POSITION_INITIAL.to_string()),
		TextFont {font: font.clone().into(), font_size: FONT_SIZE, font_smoothing: FontSmoothing::AntiAliased, ..Default::default()},
		TextColor(Color::from(FONT_COLOR))
	);

	let fps_id = commands.spawn(fps_section).insert(fps::ScreenDiagsFPS).id();
	let pos_id = commands.spawn(position_section).insert(pos::ScreenDiagsPos).id();

	commands.entity(column_id).add_child(fps_id).add_child(pos_id);
}