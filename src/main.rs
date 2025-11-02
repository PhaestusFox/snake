use bevy::{app::FixedMain, ecs::world, prelude::*};

use crate::snake::FacingDirection;

mod snake;

fn main() {
    let mut app = App::new();

    // add default plugins
    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()));

    app.add_systems(Startup, spawn_camera);

    app.add_systems(Startup, spawn_new_snake);

    app.insert_resource(Time::<Fixed>::from_hz(5.));

    app.add_plugins(snake::SnakePlugin);

    #[cfg(debug_assertions)]
    {
        app.add_systems(First, (single_step, toggle_single_step));
    }

    app.insert_resource(snake::SnakeSize(100.));

    app.run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn spawn_new_snake(mut commands: Commands) {
    commands
        .spawn((
            snake::Snake::default(),
            snake::SnakeType::BlueArrow,
            Transform::default(),
            GlobalTransform::default(),
        ))
        .with_child((snake::SnakeSegment, FacingDirection::Right))
        .with_child((snake::SnakeSegment, FacingDirection::None))
        .with_child((snake::SnakeSegment, FacingDirection::None))
        .with_child((snake::SnakeSegment, FacingDirection::None))
        .with_child((snake::SnakeSegment, FacingDirection::None));
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
            info!("Disabled single-step mode");
        } else {
            time.set_relative_speed(0.0);
            info!("Enabled single-step mode");
        }
    }
}
