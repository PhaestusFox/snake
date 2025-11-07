use bevy::{app::FixedMain, prelude::*, window::WindowResolution};

use crate::{collectables::SpawnFood, snake::FacingDirection};

mod collectables;
mod debug;
mod snake;

const U_GRID_SIZE: u32 = 32;
const WORLD_GRID_SIZE: f32 = U_GRID_SIZE as f32;

fn main() {
    let mut app = App::new();

    // add default plugins
    app.add_plugins(
        DefaultPlugins
            //use nearest-neighbor scaling for pixel art
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Sneck".to_string(),
                    resolution: utils::get_base_resolution(),
                    resize_constraints: utils::get_window_constraints(),
                    ..Default::default()
                }),
                ..Default::default()
            }),
    );

    app.add_systems(Startup, spawn_camera);

    app.add_systems(Startup, spawn_player_snake);

    app.insert_resource(Time::<Fixed>::from_hz(5.));

    app.add_plugins(snake::SnakePlugin);

    #[cfg(debug_assertions)]
    {
        app.add_plugins(debug::TestPowerPlugin);
    }

    app.insert_resource(snake::SnakeSize::Small);

    app.add_plugins(collectables::CollectablesPlugin);

    app.add_plugins(utils::idk_qol_stuff);

    app.run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn spawn_player_snake(mut commands: Commands) {
    commands
        .spawn((
            snake::Snake,
            snake::SnakeType::ArrowBlue,
            Transform::default(),
            FacingDirection::Right,
            snake::PlayerSnake,
        ))
        .with_child((snake::SnakeSegment, FacingDirection::None))
        .with_child((snake::SnakeSegment, FacingDirection::None))
        .with_child((snake::SnakeSegment, FacingDirection::None))
        .with_child((snake::SnakeSegment, FacingDirection::None))
        .with_child((snake::SnakeSegment, FacingDirection::None));

    commands.trigger(SpawnFood);
}

mod utils;
