use super::*;

#[derive(Event)]
pub struct SpawnSnake {
    skin: SnakeId,
    brain: SnakeBrain,
    length: u8,
    size: SnakeSize,
    facing: FacingDirection,
}

impl SpawnSnake {
    pub fn new(skin: SnakeId, brain: SnakeBrain, length: u8, size: SnakeSize) -> SpawnSnake {
        SpawnSnake {
            skin,
            brain,
            length: length.clamp(2, 255),
            size,
            facing: FacingDirection::Right,
        }
    }

    pub fn new_player(skin: SnakeId, length: u8, size: SnakeSize) -> SpawnSnake {
        SpawnSnake::new(skin, SnakeBrain::Player, length, size)
    }

    pub fn facing(mut self, facing: FacingDirection) -> Self {
        self.facing = facing;
        self
    }
}

pub fn spawn_snake(
    trigger: On<SpawnSnake>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let mut snake = commands.spawn((
        Snake::new(asset_server.load(trigger.skin)),
        trigger.skin,
        trigger.size,
        trigger.facing,
    ));
    match &trigger.brain {
        SnakeBrain::PathFinding(path_finding) => {
            snake.insert(path_finding.clone());
        }
        SnakeBrain::Player => {
            snake.insert(PlayerSnake);
        }
    }
    let snake_entity = snake.id();
    for _ in 0..trigger.length {
        commands.trigger(SpawnSegment {
            snake: snake_entity,
        });
    }
}

#[derive(EntityEvent)]
pub struct SpawnSegment {
    #[event_target]
    pub snake: Entity,
}

pub fn add_segment(trigger: On<SpawnSegment>, mut commands: Commands) {
    commands
        .entity(trigger.snake)
        .with_child((SnakeSegment, FacingDirection::None));
}

pub enum SnakeBrain {
    PathFinding(PathFinding),
    Player,
}
