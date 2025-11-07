use bevy::{app::FixedMain, prelude::*};

use crate::{collectables::SpawnFood, snake::FacingDirection};

mod collectables;
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
        app.add_systems(First, (single_step, toggle_single_step, change_snake));
    }

    app.insert_resource(snake::SnakeSize(100.));

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

fn single_step(world: &mut World) {
    if world
        .resource::<ButtonInput<KeyCode>>()
        .just_pressed(KeyCode::Space)
    {
        world.run_schedule(FixedMain);
    }
}

fn toggle_single_step(input: Res<ButtonInput<KeyCode>>, mut time: ResMut<Time<Virtual>>) {
    if input.just_pressed(KeyCode::F12) {
        if time.relative_speed() < 0.1 {
            time.set_relative_speed(1.0);
        } else {
            time.set_relative_speed(0.0);
        }
    }
}

fn change_snake(mut snakes: Query<&mut snake::SnakeType>, input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::F1) {
        for mut snake_type in &mut snakes {
            *snake_type = snake_type.next();
        }
    }
}
