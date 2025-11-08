use bevy::{app::FixedMain, asset::AssetPath, prelude::*, window::WindowResolution};

use sneck::{
    collectables::SpawnFood,
    snake::{FacingDirection, SnakeId, SnakeSize},
    *,
};

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

    app.add_systems(Startup, (spawn_player_snake, spawn_ai_snake));

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

fn spawn_player_snake(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            snake::Snake::new(asset_server.load(snake::SnakeId::ArrowBlue)),
            Transform::default(),
            FacingDirection::Right,
            snake::PlayerSnake,
        ))
        .with_child((snake::SnakeSegment, FacingDirection::None))
        .with_child((snake::SnakeSegment, FacingDirection::None))
        .with_child((snake::SnakeSegment, FacingDirection::None))
        .with_child((snake::SnakeSegment, FacingDirection::None))
        .with_child((snake::SnakeSegment, FacingDirection::None));

    commands.trigger(SpawnFood::Random);
    commands.trigger(SpawnFood::Random);
    commands.trigger(SpawnFood::Random);
    commands.trigger(SpawnFood::Random);
    commands.trigger(SpawnFood::Random);
    commands.trigger(SpawnFood::Random);
    commands.trigger(SpawnFood::Random);
}

fn spawn_ai_snake(mut commands: Commands, asset_server: Res<AssetServer>) {
    let snake = asset_server.load(snake::SnakeId::ArrowBlue);

    commands
        .spawn((
            snake::Snake::new(snake),
            Transform::default(),
            FacingDirection::Right,
            SnakeSize::Medium,
            snake::PathFinding::FixedPath(vec![
                FacingDirection::Up,
                FacingDirection::Left,
                FacingDirection::Down,
                FacingDirection::Right,
            ]),
        ))
        .with_child((snake::SnakeSegment, FacingDirection::None))
        .with_child((snake::SnakeSegment, FacingDirection::None))
        .with_child((snake::SnakeSegment, FacingDirection::None))
        .with_child((snake::SnakeSegment, FacingDirection::None));
}
