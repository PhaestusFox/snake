use bevy::{
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    prelude::*,
};
use leafwing_input_manager::{plugin::InputManagerSystem, prelude::*};
const SNAKE_SIZE: f32 = 25.0;
mod snake;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(InputManagerPlugin::<SnakeAction>::default())
        .init_resource::<ActionState<SnakeAction>>();
    app.add_systems(Startup, setup_world);
    app.add_systems(Startup, spawn_snake);
    app.insert_resource(Time::<Fixed>::from_hz(5.));
    app.add_systems(PreUpdate, control_snake.after(InputManagerSystem::Update));
    app.add_systems(FixedUpdate, move_snake);
    app.init_resource::<Snake>();
    app.run();
}

/// setup camera
/// setup keybindings
fn setup_world(mut commands: Commands) {
    commands.spawn(Camera2d);
    let mut input = InputMap::<SnakeAction>::default();

    input.insert(SnakeAction::MoveUp, KeyCode::KeyW);
    input.insert(SnakeAction::MoveDown, KeyCode::KeyS);
    input.insert(SnakeAction::MoveLeft, KeyCode::KeyA);
    input.insert(SnakeAction::MoveRight, KeyCode::KeyD);
    input.insert(SnakeAction::Bite, KeyCode::Space);
    input.insert(SnakeAction::MoveUp, KeyCode::ArrowUp);
    input.insert(SnakeAction::MoveDown, KeyCode::ArrowDown);
    input.insert(SnakeAction::MoveLeft, KeyCode::ArrowLeft);
    input.insert(SnakeAction::MoveRight, KeyCode::ArrowRight);
    input.insert(SnakeAction::MoveUp, GamepadButton::DPadUp);
    input.insert(SnakeAction::MoveDown, GamepadButton::DPadDown);
    input.insert(SnakeAction::MoveLeft, GamepadButton::DPadLeft);
    input.insert(SnakeAction::MoveRight, GamepadButton::DPadRight);

    commands.insert_resource(input);
}

fn spawn_snake(mut commands: Commands) {
    commands.spawn((
        Sprite {
            custom_size: Some(Vec2::new(SNAKE_SIZE, SNAKE_SIZE)),
            ..Default::default()
        },
        Segment,
        Facing::Right,
    ));
    commands.spawn((
        Sprite {
            custom_size: Some(Vec2::new(SNAKE_SIZE, SNAKE_SIZE)),
            ..Default::default()
        },
        Segment,
        Facing::Right,
        Transform::from_translation(Vec3::new(-SNAKE_SIZE, 0.0, 0.0)),
    ));
    commands.spawn((
        Sprite {
            custom_size: Some(Vec2::new(SNAKE_SIZE, SNAKE_SIZE)),
            ..Default::default()
        },
        Segment,
        Facing::Right,
        Transform::from_translation(Vec3::new(-2.0 * SNAKE_SIZE, 0.0, 0.0)),
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

fn control_snake(input: Res<ActionState<SnakeAction>>, mut query: Single<&mut Facing>) {
    println!("{:?}", input.get_just_pressed());
    for action in input.get_just_pressed() {
        match action {
            SnakeAction::MoveUp => **query = Facing::Up,
            SnakeAction::MoveDown => **query = Facing::Down,
            SnakeAction::MoveLeft => **query = Facing::Left,
            SnakeAction::MoveRight => **query = Facing::Right,
            _ => {}
        }
    }
}

#[derive(Actionlike, Reflect, Clone, Hash, PartialEq, Eq, Debug)]
enum SnakeAction {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Bite,
}

#[derive(Resource, Default, Deref, DerefMut)]
struct Snake(Vec<Entity>);

#[derive(Component)]
#[component(on_add = Self::on_add)]
struct Segment;

impl Segment {
    fn on_add(mut world: DeferredWorld, ctx: HookContext) {
        world.resource_mut::<Snake>().push(ctx.entity);
    }
}
