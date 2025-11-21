use bevy::{prelude::*, window::CursorOptions};

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
                primary_window: Some(get_basic_window()),
                primary_cursor_options: get_basic_cursor_options(),
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

        app.add_systems(Update, mouse_clicked);
    }

    app.add_plugins(collectables::CollectablesPlugin);

    app.add_plugins(utils::idk_qol_stuff);

    app.add_plugins(sneck::audio::AudioPlugin);

    app.run();
}

fn spawn_camera(mut commands: Commands) {
    let projection = OrthographicProjection::default_2d();

    commands.spawn((Camera2d, Projection::Orthographic(projection)));
}

fn spawn_player_snake(mut commands: Commands) {
    commands
        .spawn((
            SnakeId::ArrowBlue,
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

fn spawn_ai_snake(mut commands: Commands) {
    commands
        .spawn((
            SnakeId::EyeballBlueA,
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

fn get_basic_window() -> Window {
    Window {
        title: "Sneck".to_string(),
        resolution: utils::get_base_resolution(),
        resize_constraints: utils::get_window_constraints(),
        resizable: false,
        #[cfg(feature = "stream_mode")]
        transparent: true,
        #[cfg(feature = "stream_mode")]
        decorations: false,
        #[cfg(feature = "stream_mode")]
        window_level: bevy::window::WindowLevel::AlwaysOnTop,
        #[cfg(feature = "stream_mode")]
        mode: bevy::window::WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
        ..Default::default()
    }
}

fn get_basic_cursor_options() -> Option<CursorOptions> {
    // #[cfg(feature = "stream_mode")]
    // return Some(CursorOptions {
    //     hit_test: false,
    //     ..Default::default()
    // });
    // #[cfg(not(feature = "stream_mode"))]
    None
}

fn mouse_clicked(buttons: Res<ButtonInput<MouseButton>>) {
    if buttons.just_pressed(MouseButton::Left) {
        println!("Mouse left button clicked");
    }
}
