use bevy::{app::FixedMain, prelude::*};

use crate::{collectables::SpawnFood, snake::FacingDirection};

mod collectables;
mod debug;
mod snake;

fn main() {
    let mut app = App::new();

    // add default plugins
    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()));

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

    app.run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn spawn_player_snake(mut commands: Commands) {
    commands
        .spawn((
            snake::Snake,
            snake::SnakeType::BlueArrow,
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
