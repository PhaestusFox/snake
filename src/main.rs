use bevy::{log::LogPlugin, prelude::*, window::CursorOptions};

use sneck::{
    collectables::SpawnFood,
    snake::{FacingDirection, SnakeId, SnakeSize},
    ui::OpenMenu,
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
            })
            .set(LogPlugin {
                #[cfg(debug_assertions)]
                filter: "wgpu=error,naga=warn,sneck=debug".to_string(),
                ..Default::default()
            }),
    );
    app.insert_resource(bevy_pkv::PkvStore::new("PhoxGames", "Sneck"));
    app.insert_resource(ClearColor(Color::NONE));

    app.add_systems(Startup, spawn_camera);

    app.add_systems(
        OnEnter(OpenMenu::None),
        (spawn_player_snake, spawn_ai_snake),
    );

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
    app.add_plugins(sneck::ui::UiPlugin);
    app.add_plugins(sneck::SneckGame);

    app.run();
}

fn spawn_camera(mut commands: Commands) {
    let projection = OrthographicProjection::default_2d();

    commands.spawn((Camera2d, Projection::Orthographic(projection)));
}

fn spawn_player_snake(mut commands: Commands, snake_skins: Res<sneck::snake::SnakeSkins>) {
    commands.trigger(sneck::snake::SpawnSnake::new_player(
        snake_skins.active_skin(),
        5,
        SnakeSize::Small,
    ));

    commands.trigger(SpawnFood::Random);
    commands.trigger(SpawnFood::Random);
    commands.trigger(SpawnFood::Random);
    commands.trigger(SpawnFood::Random);
    commands.trigger(SpawnFood::Random);
    commands.trigger(SpawnFood::Random);
    commands.trigger(SpawnFood::Random);
}

fn spawn_ai_snake(mut commands: Commands) {
    commands.trigger(sneck::snake::SpawnSnake::new(
        SnakeId::ArrowBlue,
        sneck::snake::SnakeBrain::PathFinding(sneck::snake::PathFinding::FixedPath(vec![
            FacingDirection::Up,
            FacingDirection::Left,
            FacingDirection::Down,
            FacingDirection::Right,
        ])),
        5,
        SnakeSize::Small,
    ));
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
