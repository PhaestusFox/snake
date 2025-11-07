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
        input_map
    }
}

#[derive(Component)]
pub struct PlayerSnake;

fn update_snake_direction(
    mut snakes: Single<&mut FacingDirection, With<PlayerSnake>>,
    keys: Res<ActionState<PlayerAction>>,
) {
    for action in keys.get_just_pressed() {
        if let PlayerAction::Move(direction) = action
            && direction != snakes.invers()
        {
            **snakes = direction;
        }
    }
}
