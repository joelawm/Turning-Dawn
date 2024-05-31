use std::fmt::Write;
use bevy::{prelude::*, utils::Duration};

const POSITION_FORMAT: &str = "X:  Y:  Z: ";
pub const POSITION_INITIAL: &str = "X: ... Y: ...  Z: ...";
const POSITION_MISSING: &str = "X: ??? Y: ???  Z: ???";

#[derive(Component)]
pub struct ScreenDiagsPos;