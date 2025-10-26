use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_systems(Startup, spawn_camera);
    app.run();
}

/// Just spawn a 2d camera for now
fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
