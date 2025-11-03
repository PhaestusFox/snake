use bevy::{platform::collections::HashMap, prelude::*};
use strum::IntoEnumIterator;

use super::SnakeType;
use crate::snake::{FacingDirection, Snake, SnakeSegment, SnakeSize};

pub struct SnakeRenderPlugin;

impl Plugin for SnakeRenderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SnakeTextureHandles>()
            .add_systems(FixedPostUpdate, update_snake_texture)
            .add_systems(Update, update_snake_size);
    }
}

impl SnakeType {
    fn segment_indices(&self) -> &'static SnakePieceIndices {
        match self {
            SnakeType::WhiteSpotted => &SnakePieceIndices {
                head: 2,
                body_straight: 1,
                body_curve: 0,
                tail: 15,
            },
            SnakeType::BlueArrow => &SnakePieceIndices {
                head: 226,
                body_straight: 225,
                body_curve: 224,
                tail: 239,
            },
        }
    }

    #[inline(always)]
    fn load_snake(&self, asset_server: &AssetServer) -> SnakeHandles {
        self.segment_indices().load_snake(asset_server)
    }
}

struct SnakePieceIndices {
    head: usize,
    body_straight: usize,
    body_curve: usize,
    tail: usize,
}

impl SnakePieceIndices {
    fn load_snake(&self, asset_server: &AssetServer) -> SnakeHandles {
        SnakeHandles {
            head: asset_server.load(format!(
                "BattleSnake/snakes/32x32px_split/snake{:03}.png",
                self.head
            )),
            body_straight: asset_server.load(format!(
                "BattleSnake/snakes/32x32px_split/snake{:03}.png",
                self.body_straight
            )),
            body_curve: asset_server.load(format!(
                "BattleSnake/snakes/32x32px_split/snake{:03}.png",
                self.body_curve
            )),
            tail: asset_server.load(format!(
                "BattleSnake/snakes/32x32px_split/snake{:03}.png",
                self.tail
            )),
        }
    }
}

fn update_snake_texture(
    snakes: Query<(&Snake, &SnakeType)>,
    mut segments: Populated<(&mut Sprite, &mut Transform, &FacingDirection)>,
    textures: Res<SnakeTextureHandles>,
) {
    for (snake, snake_type) in &snakes {
        let Some(snake_images) = textures.snakes.get(snake_type) else {
            warn!("{:?} textures not loaded yet", snake_type);
            continue;
        };
        if let Some(head) = snake.first() {
            if let Ok((mut sprite, mut pos, direction)) = segments.get_mut(*head) {
                sprite.image = snake_images.head.clone();
                pos.rotation = direction.to_rotation();
                sprite.flip_x = false;
                sprite.flip_y = false;
            } else {
                warn!("Failed to get head segment sprite");
            }
        }
        for window in snake.windows(3) {
            let Ok([(_, pre, _), (mut s, mut main, _), (_, next, _)]) =
                segments.get_many_mut([window[0], window[1], window[2]])
            else {
                warn!("Failed to get middle segment sprite");
                continue;
            };
            let head = FacingDirection::moving(pre.translation, main.translation);
            let tail = FacingDirection::moving(next.translation, main.translation);
            let connection = Connection::new(head, tail);
            if connection.straight {
                s.image = textures.body_straight.clone();
            } else {
                s.image = textures.body_curve.clone();
            }
            main.rotation = Quat::from_rotation_z(connection.rotation);
            s.flip_y = connection.flip_y;
            s.flip_x = connection.flip_x;
        }
        if snake.len() > 1
            && let Some(tail) = snake.last()
            && let Some(second_last) = snake.get(snake.len() - 2)
        {
            if let Ok([(mut sprite, mut pos, direction), (_, other, _)]) =
                segments.get_many_mut([*tail, *second_last])
            {
                pos.rotation = direction.to_rotation();
                sprite.flip_x = false;
                sprite.flip_y = false;
                sprite.image = snake_images.tail.clone();
            } else {
                warn!("Failed to get tail segment sprite");
            }
        }
    }
}

fn update_snake_size(
    mut segments: Query<(&mut Sprite, &SnakeSize), Changed<SnakeSize>>,
    mut fallback_segments: Query<&mut Sprite, (Without<SnakeSize>, With<SnakeSegment>)>,
    fallback_size: Res<SnakeSize>,
) {
    for (mut sprite, size) in &mut segments {
        sprite.custom_size = Some(Vec2::splat(**size));
    }
    if fallback_size.is_changed() {
        for mut sprite in &mut fallback_segments {
            sprite.custom_size = Some(Vec2::splat(**fallback_size));
        }
    }
}

#[derive(Resource)]
struct SnakeTextureHandles {
    snakes: HashMap<SnakeType, SnakeHandles>,
}

struct SnakeHandles {
    head: Handle<Image>,
    body_straight: Handle<Image>,
    body_curve: Handle<Image>,
    tail: Handle<Image>,
}

impl SnakeTextureHandles {
    fn new(asset_server: &AssetServer) -> Self {
        let mut snakes = HashMap::new();
        for snake_type in SnakeType::iter() {
            snakes.insert(snake_type, snake_type.load_snake(asset_server));
        }
        SnakeTextureHandles { snakes }
    }
}

impl FromWorld for SnakeTextureHandles {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        SnakeTextureHandles::new(asset_server)
    }
}

struct Connection {
    flip_x: bool,
    flip_y: bool,
    straight: bool,
    rotation: f32,
}

impl Connection {
    fn new(next: FacingDirection, prev: FacingDirection) -> Self {
        match (next, prev) {
            (FacingDirection::Right, FacingDirection::Left) => Connection {
                flip_x: false,
                flip_y: false,
                straight: true,
                rotation: 0.0,
            },
            (FacingDirection::Left, FacingDirection::Right) => Connection {
                flip_x: true,
                flip_y: true,
                straight: true,
                rotation: 0.0,
            },
            (FacingDirection::Up, FacingDirection::Down) => Connection {
                flip_x: true,
                flip_y: false,
                straight: true,
                rotation: -std::f32::consts::FRAC_PI_2,
            },
            (FacingDirection::Down, FacingDirection::Up) => Connection {
                flip_x: false,
                flip_y: false,
                straight: true,
                rotation: -std::f32::consts::FRAC_PI_2,
            },
            (FacingDirection::Right, FacingDirection::Down) => Connection {
                flip_x: false,
                flip_y: false,
                straight: false,
                rotation: 0.0,
            },
            (FacingDirection::Right, FacingDirection::Up) => Connection {
                flip_x: false,
                flip_y: true,
                straight: false,
                rotation: 0.0,
            },
            (FacingDirection::Left, FacingDirection::Down) => Connection {
                flip_x: true,
                flip_y: false,
                straight: false,
                rotation: 0.0,
            },
            (FacingDirection::Left, FacingDirection::Up) => Connection {
                flip_x: true,
                flip_y: true,
                straight: false,
                rotation: 0.0,
            },
            (FacingDirection::Down, FacingDirection::Right) => Connection {
                flip_x: true,
                flip_y: false,
                straight: false,
                rotation: std::f32::consts::FRAC_PI_2,
            },
            (FacingDirection::Down, FacingDirection::Left) => Connection {
                flip_x: false,
                flip_y: false,
                straight: false,
                rotation: -std::f32::consts::FRAC_PI_2,
            },
            (FacingDirection::Up, FacingDirection::Right) => Connection {
                flip_x: false,
                flip_y: false,
                straight: false,
                rotation: std::f32::consts::FRAC_PI_2,
            },
            (FacingDirection::Up, FacingDirection::Left) => Connection {
                flip_x: true,
                flip_y: false,
                straight: false,
                rotation: -std::f32::consts::FRAC_PI_2,
            },
            _ => {
                warn!("Not implemented: {next:?} {prev:?}");
                Connection {
                    flip_x: false,
                    flip_y: false,
                    straight: true,
                    rotation: 0.0,
                }
            }
        }
    }
}
