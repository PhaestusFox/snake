use std::marker::PhantomData;

use bevy::{platform::collections::HashMap, prelude::*};
use strum::IntoEnumIterator;

use super::*;
use crate::snake::{FacingDirection, Snake, SnakeSize};

pub struct SnakeRenderPlugin;

impl Plugin for SnakeRenderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SnakeTextureHandles>()
            .add_systems(
                Update,
                (update_snake_animation_frame, update_snake_texture).chain(),
            )
            .add_systems(
                FixedUpdate,
                update_snake_shape.after(super::movement::move_snake),
            )
            .add_systems(Update, (update_snake_size, change_skin));

        app.init_resource::<AnimationTime>();
    }
}

impl SnakeId {
    pub fn segment_indices(&self) -> &'static SnakePieceIndices {
        match self {
            SnakeId::SpottedWhite => &SnakePieceIndices {
                head: 2,
                body_straight: 1,
                body_curve: 0,
                tail: 15,
            },
            SnakeId::ArrowBlue => &SnakePieceIndices {
                head: 226,
                body_straight: 225,
                body_curve: 224,
                tail: 239,
            },
            SnakeId::GearWindowA => &SnakePieceIndices {
                head: 338,
                body_straight: 336,
                body_curve: 340,
                tail: 342,
            },
            SnakeId::GearRickA => &SnakePieceIndices {
                head: 344,
                body_straight: 336,
                body_curve: 340,
                tail: 342,
            },
            SnakeId::GearPreWindowA => &SnakePieceIndices {
                head: 346,
                body_straight: 336,
                body_curve: 340,
                tail: 342,
            },
            SnakeId::EyeballBlueA => &SnakePieceIndices {
                head: 370,
                body_straight: 368,
                body_curve: 374,
                tail: 372,
            },
            SnakeId::EyeballYellowA => &SnakePieceIndices {
                head: 378,
                body_straight: 376,
                body_curve: 382,
                tail: 380,
            },
        }
    }

    #[inline(always)]
    fn load_snake(&self, asset_server: &AssetServer) -> SnakeHandles {
        self.segment_indices().load_snake(asset_server)
    }
}

pub struct SnakePieceIndices {
    pub head: usize,
    pub body_straight: usize,
    pub body_curve: usize,
    pub tail: usize,
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
    snakes: Query<(&Snake, &Children)>,
    textures: Res<Assets<SnakeType>>,
    mut segments: Query<(&mut Sprite, &SnakePiece)>,
) {
    for (snake, children) in &snakes {
        let Some(snake_images) = textures.get(snake) else {
            warn!("{} textures not loaded yet", snake);
            continue;
        };
        for child in children.iter() {
            if let Ok((mut sprite, piece)) = segments.get_mut(child) {
                sprite.image = snake_images.get_frame(snake.frame).get_segment(piece);
            }
        }
    }
}

fn update_snake_shape(
    snakes: Query<(Entity, &Children), With<Snake>>,
    mut segments: Populated<(
        &mut Sprite,
        &mut Transform,
        &mut SnakePiece,
        &FacingDirection,
    )>,
) {
    for (root, body) in &snakes {
        if body.is_empty() {
            warn!("Snake {:?} has no segments", root);
            continue;
        }
        if let Ok((mut sprite, mut pos, mut piece, direction)) = segments.get_mut(body[0]) {
            *piece = SnakePiece::Head;
            pos.rotation = direction.to_rotation();
            sprite.flip_x = false;
            sprite.flip_y = false;
        } else {
            warn!("Failed to get head segment sprite");
        }
        if body.len() > 2 {
            for window in body.windows(2).skip(1) {
                let Ok([(mut s, mut main, mut piece, head), (.., tail)]) =
                    segments.get_many_mut([window[0], window[1]])
                else {
                    warn!("Failed to get middle segment sprite");
                    continue;
                };
                // let head = FacingDirection::moving(pre.translation, main.translation);
                // let tail = FacingDirection::moving(next.translation, main.translation);
                let connection = Connection::new(*head, *tail);
                if connection.straight {
                    *piece = SnakePiece::BodyStraight;
                } else {
                    *piece = SnakePiece::BodyCurve;
                }
                main.rotation = Quat::from_rotation_z(connection.rotation);
                s.flip_y = connection.flip_y;
                s.flip_x = connection.flip_x;
            }
        }

        if body.len() > 1
            && let Some(tail) = body.last()
        {
            if let Ok((mut sprite, mut pos, mut piece, direction)) = segments.get_mut(*tail) {
                pos.rotation = direction.to_rotation();
                sprite.flip_x = false;
                sprite.flip_y = false;
                *piece = SnakePiece::Tail;
            } else {
                warn!("Failed to get tail segment sprite");
            }
        }
    }
}

fn update_snake_size(
    mut snakes: Populated<(&Children, &SnakeSize), Changed<SnakeSize>>,
    mut sprites: Query<&mut Sprite>,
) {
    for (body, size) in &mut snakes {
        let size = **size;
        for segment in body.iter() {
            if let Ok(mut sprite) = sprites.get_mut(segment) {
                sprite.custom_size = Some(Vec2::splat(size));
            }
        }
    }
}

#[derive(Resource, Deref)]
pub struct SnakeTextureHandles {
    snakes: HashMap<SnakeId, SnakeHandles>,
}

pub struct SnakeHandles {
    pub head: Handle<Image>,
    pub body_straight: Handle<Image>,
    pub body_curve: Handle<Image>,
    pub tail: Handle<Image>,
}

impl SnakeHandles {
    pub fn get(&self, piece: &SnakePiece) -> Handle<Image> {
        match piece {
            SnakePiece::Head => self.head.clone(),
            SnakePiece::BodyStraight => self.body_straight.clone(),
            SnakePiece::BodyCurve => self.body_curve.clone(),
            SnakePiece::Tail => self.tail.clone(),
        }
    }
}

impl SnakeTextureHandles {
    fn new(asset_server: &AssetServer) -> Self {
        let mut snakes = HashMap::new();
        for snake_type in SnakeId::iter() {
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
            (FacingDirection::Right, FacingDirection::Right) => Connection {
                flip_x: false,
                flip_y: false,
                straight: true,
                rotation: 0.0,
            },
            (FacingDirection::Left, FacingDirection::Left) => Connection {
                flip_x: true,
                flip_y: true,
                straight: true,
                rotation: 0.0,
            },
            (FacingDirection::Up, FacingDirection::Up) => Connection {
                flip_x: true,
                flip_y: false,
                straight: true,
                rotation: -std::f32::consts::FRAC_PI_2,
            },
            (FacingDirection::Down, FacingDirection::Down) => Connection {
                flip_x: false,
                flip_y: false,
                straight: true,
                rotation: -std::f32::consts::FRAC_PI_2,
            },
            (FacingDirection::Right, FacingDirection::Up) => Connection {
                flip_x: false,
                flip_y: false,
                straight: false,
                rotation: 0.0,
            },
            (FacingDirection::Right, FacingDirection::Down) => Connection {
                flip_x: false,
                flip_y: true,
                straight: false,
                rotation: 0.0,
            },
            (FacingDirection::Left, FacingDirection::Up) => Connection {
                flip_x: true,
                flip_y: false,
                straight: false,
                rotation: 0.0,
            },
            (FacingDirection::Left, FacingDirection::Down) => Connection {
                flip_x: true,
                flip_y: true,
                straight: false,
                rotation: 0.0,
            },
            (FacingDirection::Down, FacingDirection::Left) => Connection {
                flip_x: true,
                flip_y: false,
                straight: false,
                rotation: std::f32::consts::FRAC_PI_2,
            },
            (FacingDirection::Down, FacingDirection::Right) => Connection {
                flip_x: false,
                flip_y: false,
                straight: false,
                rotation: -std::f32::consts::FRAC_PI_2,
            },
            (FacingDirection::Up, FacingDirection::Left) => Connection {
                flip_x: false,
                flip_y: false,
                straight: false,
                rotation: std::f32::consts::FRAC_PI_2,
            },
            (FacingDirection::Up, FacingDirection::Right) => Connection {
                flip_x: true,
                flip_y: false,
                straight: false,
                rotation: -std::f32::consts::FRAC_PI_2,
            },
            _ => {
                // warn!("Not implemented: {next:?} {prev:?}");
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

#[derive(Resource, Deref)]
struct AnimationTime(f32);

impl FromWorld for AnimationTime {
    fn from_world(_world: &mut World) -> Self {
        AnimationTime(1. / 8.) // Change animation frame every 8 updates
    }
}

fn update_snake_animation_frame(
    animation_time: Res<AnimationTime>,
    mut accrued: Local<f32>,
    mut snakes: Query<&mut Snake>,
    time: Res<Time<Real>>,
) {
    *accrued += time.delta_secs();
    if *accrued > animation_time.0 {
        *accrued -= animation_time.0;
    } else {
        return;
    }
    for mut snake in &mut snakes {
        snake.frame += 1;
    }
}

pub enum SnakeBody {
    Animated(Vec<SnakeFrame>),
    Single(SnakeFrame),
}

impl SnakeBody {
    /// Get the tail handle for this body<br/>
    /// If animated, returns the tail of the first frame
    pub fn tail(&self) -> Handle<Image> {
        match self {
            SnakeBody::Animated(frames) => frames[0].tail.clone(),
            SnakeBody::Single(frame) => frame.tail.clone(),
        }
    }
}

pub struct SnakeFrame {
    pub head: Handle<Image>,
    pub body_straight: Handle<Image>,
    pub body_curve: Handle<Image>,
    pub tail: Handle<Image>,
}

impl SnakeFrame {
    pub const EMPTY: Self = SnakeFrame {
        head: Handle::Uuid(AssetId::<Image>::DEFAULT_UUID, PhantomData),
        body_straight: Handle::Uuid(AssetId::<Image>::DEFAULT_UUID, PhantomData),
        body_curve: Handle::Uuid(AssetId::<Image>::DEFAULT_UUID, PhantomData),
        tail: Handle::Uuid(AssetId::<Image>::DEFAULT_UUID, PhantomData),
    };

    pub fn get_segment(&self, piece: &SnakePiece) -> Handle<Image> {
        match piece {
            SnakePiece::Head => self.head.clone(),
            SnakePiece::BodyStraight => self.body_straight.clone(),
            SnakePiece::BodyCurve => self.body_curve.clone(),
            SnakePiece::Tail => self.tail.clone(),
        }
    }
}

fn change_skin(
    mut changed: Query<(&mut Snake, &SnakeId), Changed<SnakeId>>,
    asset_server: Res<AssetServer>,
) {
    for (mut snake, snake_id) in &mut changed {
        snake.snake_type = asset_server.load(*snake_id);
    }
}
