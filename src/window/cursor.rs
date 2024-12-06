use bevy::{prelude::*, window::{CursorGrabMode, PrimaryWindow}};

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Cursor>()
            .add_systems(Update, update_cursor_locking)
            .add_systems(Update, lock_cursor_position)
            .add_systems(Startup, init_cursor_properties);
    }
}

#[derive(Resource, Default)]
pub struct Cursor {
    locked: bool,
}

impl Cursor {
    /// This inverts the cursor lock
    pub fn invert_lock(&mut self, window: &mut Mut<'_, Window>) {
        self.locked = !self.locked;
        window.cursor_options.visible = !self.locked;

        if self.locked {
            let window_width = window.width();
            let window_height = window.height();
            window.cursor_options.grab_mode = CursorGrabMode::Locked;
            window.set_cursor_position(Some(Vec2::new(window_width / 2., window_height / 2.)));
        } else {
            window.cursor_options.grab_mode = CursorGrabMode::None;
        }
    }
}

/// This locks the cursor to the center of the window
fn lock_cursor_position(mut window_query: Query<&mut Window, With<PrimaryWindow>>, cursor: ResMut<Cursor>) {
    let mut window = window_query.get_single_mut().unwrap();

    if !cursor.locked {
        return;
    }

    let window_width = window.width();
    let window_height = window.height();
    window.set_cursor_position(Some(Vec2::new(window_width / 2., window_height / 2.)));
}

/// This initializes the cursor properties
fn init_cursor_properties(mut window_query: Query<&mut Window, With<PrimaryWindow>>, mut cursor: ResMut<Cursor>) {
    let mut window = window_query.get_single_mut().unwrap();

    cursor.invert_lock(&mut window);
}

/// This updates the cursor locking
fn update_cursor_locking(keys: Res<ButtonInput<KeyCode>>, mut window_query: Query<&mut Window, With<PrimaryWindow>>, mut cursor: ResMut<Cursor>) {
    let mut window = window_query.get_single_mut().unwrap();

    if keys.just_pressed(KeyCode::Escape) {
        cursor.invert_lock(&mut window);
    }
}