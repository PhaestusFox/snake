use bevy::prelude::*;

use sneck::*;

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
                    transparent: true,
                    // decorations: false,
                    window_level: bevy::window::WindowLevel::AlwaysOnTop,
                    ..Default::default()
                }),
                ..Default::default()
            }),
    );
    app.insert_resource(ClearColor(Color::NONE));

    app.add_systems(Startup, spawn_camera);

    app.insert_resource(Time::<Fixed>::from_hz(5.));

    app.add_plugins((snake::SnakePlugin, map::MapPlugin));

    #[cfg(debug_assertions)]
    {
        app.add_plugins(debug::TestPowerPlugin);
    }

    app.add_plugins(collectables::CollectablesPlugin);

    app.add_plugins(utils::idk_qol_stuff);

    app.run();
}

fn spawn_camera(mut commands: Commands) {
    let projection = OrthographicProjection::default_2d();

    commands.spawn((Camera2d, Projection::Orthographic(projection)));
}
