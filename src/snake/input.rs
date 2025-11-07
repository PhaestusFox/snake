use super::*;
use leafwing_input_manager::prelude::*;
pub struct SnakeInputPlugin;

impl Plugin for SnakeInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlayerAction>::default())
            .insert_resource(PlayerAction::get_default_keybindings())
            .init_resource::<ActionState<PlayerAction>>()
            .add_systems(
                PreUpdate,
                update_snake_direction
                    .after(leafwing_input_manager::plugin::InputManagerSystem::Update),
            );
    }
}

#[derive(leafwing_input_manager::Actionlike, Clone, Copy, PartialEq, Eq, Hash, Debug, Reflect)]
pub enum PlayerAction {
    Reset,
    Move(FacingDirection),
}

impl PlayerAction {
    fn get_default_keybindings() -> InputMap<PlayerAction> {
        let mut input_map = InputMap::default();
        // WASD for movement
        input_map.insert(PlayerAction::Move(FacingDirection::Up), KeyCode::KeyW);
        input_map.insert(PlayerAction::Move(FacingDirection::Down), KeyCode::KeyS);
        input_map.insert(PlayerAction::Move(FacingDirection::Left), KeyCode::KeyA);
        input_map.insert(PlayerAction::Move(FacingDirection::Right), KeyCode::KeyD);
        // Arrow keys for movement
        input_map.insert(PlayerAction::Move(FacingDirection::Up), KeyCode::ArrowUp);
        input_map.insert(
            PlayerAction::Move(FacingDirection::Down),
            KeyCode::ArrowDown,
        );
        input_map.insert(
            PlayerAction::Move(FacingDirection::Left),
            KeyCode::ArrowLeft,
        );
        input_map.insert(
            PlayerAction::Move(FacingDirection::Right),
            KeyCode::ArrowRight,
        );

        // Gamepad buttons for movement
        input_map.insert(
            PlayerAction::Move(FacingDirection::Up),
            GamepadButton::North,
        );
        input_map.insert(
            PlayerAction::Move(FacingDirection::Down),
            GamepadButton::South,
        );
        input_map.insert(
            PlayerAction::Move(FacingDirection::Left),
            GamepadButton::West,
        );
        input_map.insert(
            PlayerAction::Move(FacingDirection::Right),
            GamepadButton::East,
        );

        // Gamepad axes for movement
        input_map.insert(
            PlayerAction::Move(FacingDirection::Up),
            GamepadControlDirection::positive(GamepadAxis::LeftStickY),
        );
        input_map.insert(
            PlayerAction::Move(FacingDirection::Down),
            GamepadControlDirection::negative(GamepadAxis::LeftStickY),
        );
        input_map.insert(
            PlayerAction::Move(FacingDirection::Left),
            GamepadControlDirection::negative(GamepadAxis::LeftStickX),
        );
        input_map.insert(
            PlayerAction::Move(FacingDirection::Right),
            GamepadControlDirection::positive(GamepadAxis::LeftStickX),
        );

        input_map
    }
}

#[derive(Component)]
pub struct PlayerSnake;

fn update_snake_direction(
    mut snakes: Single<(&mut FacingDirection, &Children), With<PlayerSnake>>,
    segments: Query<&FacingDirection, Without<PlayerSnake>>,
    keys: Res<ActionState<PlayerAction>>,
) {
    for action in keys.get_just_pressed() {
        let Ok(head) = segments.get(snakes.1[0]) else {
            warn!("Player Snake has no head?");
            continue;
        };
        if let PlayerAction::Move(direction) = action
            && direction != head.invers()
        {
            *snakes.0 = direction;
        }
    }
}
