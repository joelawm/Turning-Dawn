use std::fmt::Write;
use bevy::{diagnostic::{FrameTimeDiagnosticsPlugin, DiagnosticsStore}, prelude::*, utils::Duration};

const FONT_SIZE: f32 = 18.0;
const FONT_COLOR: Color = Color::WHITE;
const UPDATE_INTERVAL: Duration = Duration::from_secs(1);

const STRING_FORMAT: &str = "FPS: ";
const STRING_INITIAL: &str = "FPS: ...";
const STRING_MISSING: &str = "FPS: ???";

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
        app.add_plugins(ScreenDiagsPlugin)
            .add_systems(Startup, spawn_text)
            .add_systems(Update, update);
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
        Self {
            timer: Timer::new(UPDATE_INTERVAL, TimerMode::Repeating),
            update_now: true,
        }
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

/// The marker on the text to be updated.
#[derive(Component)]
pub struct ScreenDiagsText;

fn update(time: Res<Time>, diagnostics: Res<DiagnosticsStore>, state_resource: Option<ResMut<ScreenDiagsState>>, mut text_query: Query<&mut Text, With<ScreenDiagsText>>) {
    if let Some(mut state) = state_resource {
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
                    let value = &mut text.sections[0].value;
                    value.clear();

                    if let Some(fps) = fps_diags {
                        write!(value, "{}{:.0}", STRING_FORMAT, fps).unwrap();
                    } else {
                        value.clear();
                        write!(value, "{}", STRING_MISSING).unwrap();
                    }
                }
            }
        }
    }
}

/// Get the current fps
pub fn extract_fps(diagnostics: &Res<DiagnosticsStore>) -> Option<f64> {
    diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS).and_then(|fps| fps.average())
}

fn spawn_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("font/screen-diags-font.ttf");
    commands
        .spawn(TextBundle {
            text: Text {
                sections: vec![TextSection {
                    value: STRING_INITIAL.to_string(),
                    style: TextStyle {
                        font,
                        font_size: FONT_SIZE,
                        color: FONT_COLOR,
                    },
                }],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(ScreenDiagsText);
}