use bevy::window::{WindowResized, WindowResolution};

use super::*;

pub fn idk_qol_stuff(app: &mut App) {
    app.add_systems(Update, force_window_resilution_to_have_factor_of_cell_size);
    app.add_systems(Startup, spawn_play_area_marker);
}

fn force_window_resilution_to_have_factor_of_cell_size(
    mut resize_messages: MessageReader<WindowResized>,
    mut windows: Query<&mut Window>,
) {
    for resize in resize_messages.read() {
        let Ok(mut window) = windows.get_mut(resize.window) else {
            warn!(
                "Failed to get window({}) to enforce resolution",
                resize.window
            );
            continue;
        };
        let old = window.size().as_uvec2();
        let new = old / U_GRID_SIZE * U_GRID_SIZE;
        if old == new {
            continue;
        }
        window.resolution.set(new.x as f32, new.y as f32);
    }
}

// const RESOLUTION: (u32, u32) = (U_GRID_SIZE * 4 * 8 * 2, U_GRID_SIZE * 3 * 8 * 2);
const RESOLUTION: (u32, u32) = (U_GRID_SIZE * 100, U_GRID_SIZE * 75);
// Minimum window size is one Segment of the Biggest SnakeSize
const MIN_WINDOW_SIZE: Vec2 = Vec2::new(GRID_SIZE * 8., GRID_SIZE * 8.);

pub fn get_base_resolution() -> WindowResolution {
    WindowResolution::new(RESOLUTION.0, RESOLUTION.1)
}

pub fn get_window_constraints() -> WindowResizeConstraints {
    WindowResizeConstraints {
        min_width: MIN_WINDOW_SIZE.x,
        min_height: MIN_WINDOW_SIZE.y,
        max_width: f32::INFINITY,
        max_height: f32::INFINITY,
    }
}

fn spawn_play_area_marker(mut commands: Commands) {
    commands.spawn((
        Transform::from_translation(Vec3::new(0.0, 0.0, -100.0)),
        Sprite {
            color: Color::linear_rgb(0.0, 0.3, 0.0),
            custom_size: Some(Vec2::new(RESOLUTION.0 as f32, RESOLUTION.1 as f32)),
            ..Default::default()
        },
    ));
}
