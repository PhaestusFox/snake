use std::ops::Deref;

use bevy::{
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    prelude::*,
};

mod input;
mod movement;
mod rendering;

pub use input::PlayerSnake;

use crate::WORLD_GRID_SIZE;

#[derive(Component)]
#[require(Transform, SnakeType, Visibility)]
pub struct Snake;

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(rendering::SnakeRenderPlugin)
            .add_plugins(movement::plugin)
            .init_resource::<SnakeSize>();

        app.add_plugins(input::SnakeInputPlugin);
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
#[require(Transform, FacingDirection, Sprite)]
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
            let snake_type = world
                .get::<SnakeType>(parent)
                .copied()
                .unwrap_or(SnakeType::default());
            // get the textures for this snake type
            let textures = world
                .resource::<rendering::SnakeTextureHandles>()
                .get(&snake_type)
                // this will *probably* be the tail so grab that texture
                .map(|c| c.tail.clone());

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
            world
                .commands()
                .spawn((Snake, transform))
                .add_child(ctx.entity)
                .with_child(SnakeSegment);
        };
    }
}

#[derive(
    Clone,
    Copy,
    Component,
    Default,
    PartialEq,
    Eq,
    Hash,
    strum_macros::EnumIter,
    Debug,
    strum_macros::FromRepr,
)]
pub enum SnakeType {
    #[default]
    WhiteSpotted = 0,
    BlueArrow = 14,
}

impl SnakeType {
    pub fn next(&self) -> Self {
        let mut iter = <SnakeType as strum::IntoEnumIterator>::iter();
        for variant in iter.by_ref() {
            if &variant == self {
                return iter.next().unwrap_or(SnakeType::WhiteSpotted);
            }
        }
        SnakeType::WhiteSpotted
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
