use std::ops::Deref;

use bevy::{
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    prelude::*,
    sprite::Anchor,
};

mod input;
mod movement;
mod rendering;

pub use input::PlayerSnake;

use crate::WORLD_GRID_SIZE;

#[derive(Component, DerefMut, Deref, Clone)]
#[require(Transform, Visibility, FacingDirection)]
pub struct Snake {
    #[deref]
    pub snake_type: Handle<SnakeType>,
    pub frame: usize,
}

impl From<&Snake> for AssetId<SnakeType> {
    fn from(value: &Snake) -> Self {
        value.snake_type.id()
    }
}

impl std::fmt::Display for Snake {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.snake_type {
            Handle::Uuid(id, _) => write!(f, "Snake({id})"),
            Handle::Strong(ref id) => {
                let Ok(Some(path)) = id.path::<Option<bevy::asset::AssetPath<'static>>>("path")
                else {
                    return write!(f, "Snake(Unknown)");
                };
                write!(f, "{:?} Snake", path.path().file_name())
            }
        }
    }
}

impl Snake {
    pub fn new(snake_type: Handle<SnakeType>) -> Self {
        Snake {
            snake_type,
            frame: 0,
        }
    }
}

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(rendering::SnakeRenderPlugin)
            .add_plugins(movement::plugin)
            .init_resource::<SnakeSize>();

        app.init_asset_loader::<asset::SnakeLoader>();
        app.init_asset::<asset::SnakeType>();

        app.add_plugins(input::SnakeInputPlugin);

        app.add_plugins(pathing::plugin);
    }
}

#[derive(Component, Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum FacingDirection {
    Right,
    Down,
    Left,
    Up,
    #[default]
    None,
}

impl FacingDirection {
    pub fn to_rotation(self) -> Quat {
        match self {
            FacingDirection::Right => Quat::IDENTITY,
            FacingDirection::Down => Quat::from_rotation_z(-std::f32::consts::FRAC_PI_2),
            FacingDirection::Left => Quat::from_rotation_z(std::f32::consts::PI),
            FacingDirection::Up => Quat::from_rotation_z(std::f32::consts::FRAC_PI_2),
            FacingDirection::None => Quat::IDENTITY,
        }
    }

    pub fn to_vec(self) -> Vec3 {
        match self {
            FacingDirection::Right => Vec3::new(1.0, 0.0, 0.0),
            FacingDirection::Down => Vec3::new(0.0, -1.0, 0.0),
            FacingDirection::Left => Vec3::new(-1.0, 0.0, 0.0),
            FacingDirection::Up => Vec3::new(0.0, 1.0, 0.0),
            FacingDirection::None => Vec3::ZERO,
        }
    }

    pub fn invers(&self) -> Self {
        match self {
            FacingDirection::Right => FacingDirection::Left,
            FacingDirection::Down => FacingDirection::Up,
            FacingDirection::Left => FacingDirection::Right,
            FacingDirection::Up => FacingDirection::Down,
            FacingDirection::None => FacingDirection::None,
        }
    }
}

#[derive(Component, Default)]
#[component(on_add = Self::on_add)]
#[require(Transform, FacingDirection, Sprite, SnakePiece)]
pub struct SnakeSegment;

impl SnakeSegment {
    fn on_add(mut world: DeferredWorld, ctx: HookContext) {
        if let Some(&ChildOf(parent)) = world.get::<ChildOf>(ctx.entity) {
            // get children of parent snake
            if let Some(sibling) = world.get::<Children>(parent) &&
            // get last child; the end of the snake
            let Some(pre) = sibling.last().cloned()
            {
                // get the index of this segment in the siblings
                let layer = sibling.len();

                // get transform of last segment
                if let Some(pos) = world.get::<Transform>(pre).copied()
                && let Some(facing) = world.get::<FacingDirection>(pre).copied()
                // get transform of this segment to modify
                && let Some(mut seg_transform) = world.get_mut::<Transform>(ctx.entity)
                {
                    seg_transform.translation = pos.translation;
                    seg_transform.translation.z = -(layer as f32);
                    seg_transform.rotation = facing.to_rotation();
                }
            }

            // get the size of the snake from the parent, or fallback to global resource
            let size = if let Some(size) = world.get::<SnakeSize>(parent) {
                **size
            } else {
                **world.resource::<SnakeSize>()
            };

            // get the snake type from the parent to determine textures
            let textures = if let Some(snake_type) = world.get::<Snake>(parent) {
                // get the textures for this snake type
                world
                    .resource::<Assets<SnakeType>>()
                    .get(snake_type)
                    // this will *probably* be the tail so grab that texture
                    .map(|c| c.body.tail())
            } else {
                None
            };

            // set the size of the segment sprite
            let mut sprite = world
                .get_mut::<Sprite>(ctx.entity)
                .expect("SnakeSegment requires Sprite");
            sprite.custom_size = Some(Vec2::splat(size));
            if let Some(tail) = textures {
                sprite.image = tail;
            }
        } else {
            info!(
                "SnakeSegment added to entity that is not a child of a Snake;\nMitosis ACTIVATED;\nspawning new Snake;"
            );
            let transform = world
                .get::<Transform>(ctx.entity)
                .cloned()
                .unwrap_or_default();
            let default_snake = world.resource::<AssetServer>().load(SnakeId::default());
            world
                .commands()
                .spawn((Snake::new(default_snake), transform))
                .add_child(ctx.entity)
                .with_child(SnakeSegment);
        };
    }
}

#[derive(
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq,
    Hash,
    strum_macros::EnumIter,
    Debug,
    strum_macros::FromRepr,
    strum_macros::IntoStaticStr,
)]
pub enum SnakeId {
    #[default]
    SpottedWhite = 0,
    ArrowBlue,
    GearWindowA,
    GearRickA,
    GearPreWindowA,
    EyeballBlueA,
    EyeballYellowA,
}

impl From<SnakeId> for bevy::asset::AssetPath<'static> {
    fn from(value: SnakeId) -> bevy::asset::AssetPath<'static> {
        bevy::asset::AssetPath::from(format!("snakes/{value:?}.snake"))
    }
}

impl SnakeId {
    pub fn next(&self) -> Self {
        let mut iter = <SnakeId as strum::IntoEnumIterator>::iter();
        for id in iter.by_ref() {
            if &id == self {
                return iter.next().unwrap_or(SnakeId::default());
            }
        }
        SnakeId::default()
    }
}

#[derive(Resource, Component, Clone, Copy, PartialEq, Eq, Default)]
pub enum SnakeSize {
    #[default]
    Small,
    Medium,
    Large,
    Colossal,
}

impl AsRef<f32> for SnakeSize {
    fn as_ref(&self) -> &f32 {
        match self {
            SnakeSize::Small => &WORLD_GRID_SIZE,
            SnakeSize::Medium => &(WORLD_GRID_SIZE * 2.0),
            SnakeSize::Large => &(WORLD_GRID_SIZE * 4.0),
            SnakeSize::Colossal => &(WORLD_GRID_SIZE * 8.0),
        }
    }
}

impl Deref for SnakeSize {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl SnakeSize {
    pub fn up(&self) -> Self {
        match self {
            SnakeSize::Small => SnakeSize::Medium,
            SnakeSize::Medium => SnakeSize::Large,
            SnakeSize::Large => SnakeSize::Colossal,
            SnakeSize::Colossal => SnakeSize::Colossal,
        }
    }

    pub fn down(&self) -> Self {
        match self {
            SnakeSize::Small => SnakeSize::Small,
            SnakeSize::Medium => SnakeSize::Small,
            SnakeSize::Large => SnakeSize::Medium,
            SnakeSize::Colossal => SnakeSize::Large,
        }
    }

    pub fn inc(&mut self) {
        *self = self.up();
    }
    pub fn dec(&mut self) {
        *self = self.down();
    }
}

#[derive(Component, Default)]
enum SnakePiece {
    Head,
    BodyStraight,
    BodyCurve,
    #[default]
    Tail,
}

mod asset;

pub use asset::SnakeType;

mod pathing;
pub use pathing::PathFinding;
