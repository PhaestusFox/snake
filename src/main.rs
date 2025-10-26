use bevy::prelude::*;
const SNAKE_SIZE: f32 = 25.0;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_systems(Startup, spawn_camera);
    app.add_systems(Startup, spawn_snake);
    app.insert_resource(Time::<Fixed>::from_hz(5.));
    app.add_systems(PreUpdate, control_snake);
    app.add_systems(FixedUpdate, move_snake);
    app.run();
}

/// Just spawn a 2d camera for now
fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn spawn_snake(mut commands: Commands) {
    commands.spawn((
        Sprite {
            custom_size: Some(Vec2::new(SNAKE_SIZE, SNAKE_SIZE)),
            ..Default::default()
        },
        Facing::Right,
    ));
}

#[derive(Component)]
enum Facing {
    Up,
    Down,
    Left,
    Right,
}

fn move_snake(mut snake: Query<(&mut Transform, &Facing)>) {
    for (mut transform, facing) in &mut snake {
        match facing {
            Facing::Up => transform.translation.y += SNAKE_SIZE,
            Facing::Down => transform.translation.y -= SNAKE_SIZE,
            Facing::Left => transform.translation.x -= SNAKE_SIZE,
            Facing::Right => transform.translation.x += SNAKE_SIZE,
        }
    }
}

fn control_snake(input: Res<ButtonInput<KeyCode>>, mut query: Single<&mut Facing>) {
    for key in input.get_just_pressed() {
        match key {
            KeyCode::ArrowUp | KeyCode::KeyW => **query = Facing::Up,
            KeyCode::KeyS => **query = Facing::Down,
            KeyCode::KeyA => **query = Facing::Left,
            KeyCode::KeyD => **query = Facing::Right,
            _ => {}
        }
    }
}
