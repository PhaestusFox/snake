pub mod collectables;
pub mod debug;
pub mod map;
pub mod snake;
pub mod utils;

use bevy::prelude::*;

pub const U_GRID_SIZE: u32 = 16;
pub const GRID_SIZE: f32 = U_GRID_SIZE as f32;

#[cfg(feature = "streamer_mode")]
pub mod streamer_mode;
