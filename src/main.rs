use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_systems(Startup, spawn_camera);
    app.add_systems(Startup, spawn_snake);
    app.run();
}

/// Just spawn a 2d camera for now
fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn spawn_snake(mut commands: Commands) {
    commands.spawn(Sprite {
        custom_size: Some(Vec2::new(10.0, 10.0)),
        ..Default::default()
    });
}
