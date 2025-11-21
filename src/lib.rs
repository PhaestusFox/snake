#![feature(if_let_guard)]

pub mod audio;
pub mod collectables;
pub mod debug;
pub mod map;
pub mod snake;
pub mod ui;
pub mod utils;

use bevy::prelude::*;

pub const U_GRID_SIZE: u32 = 16;
pub const GRID_SIZE: f32 = U_GRID_SIZE as f32;

#[cfg(feature = "streamer_mode")]
pub mod streamer_mode;

#[derive(Default, Debug, Hash, PartialEq, Eq, Clone)]
struct TimeFrozen;

impl ComputedStates for TimeFrozen {
    type SourceStates = ui::OpenMenu;
    fn compute(sources: Self::SourceStates) -> Option<Self> {
        (!matches!(sources, ui::OpenMenu::None)).then_some(Self)
    }
}

pub struct SneckGame;

impl Plugin for SneckGame {
    fn build(&self, app: &mut App) {
        app.add_computed_state::<TimeFrozen>();
        app.add_systems(OnEnter(TimeFrozen), freeze_time);
        app.add_systems(OnExit(TimeFrozen), unfreeze_time);
    }
}

fn freeze_time(mut time: ResMut<Time<Virtual>>) {
    debug!("freezing time");
    time.pause();
}

fn unfreeze_time(mut time: ResMut<Time<Virtual>>) {
    debug!("unfreezing time");
    time.unpause();
}
