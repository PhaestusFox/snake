use bevy::{
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    prelude::*,
};

mod movement;
mod rendering;

const SNAKE_SIZE: f32 = 32.0;

#[derive(Component)]
#[require(Transform, SnakeType, Visibility)]
pub struct Snake;

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(rendering::SnakeRenderPlugin)
            .add_plugins(movement::plugin)
            .add_systems(PreUpdate, user_input)
            .init_resource::<SnakeSize>();
    }
}

#[derive(Component, Default, Debug, Clone, Copy, PartialEq, Eq)]
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
}

fn user_input(
    mut snakes: Query<&mut FacingDirection, With<Snake>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for mut facing in snakes.iter_mut() {
        if keys.pressed(KeyCode::KeyD) {
            *facing = FacingDirection::Right;
        } else if keys.pressed(KeyCode::KeyS) {
            *facing = FacingDirection::Down;
        } else if keys.pressed(KeyCode::KeyA) {
            *facing = FacingDirection::Left;
        } else if keys.pressed(KeyCode::KeyW) {
            *facing = FacingDirection::Up;
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
            if world.get::<Snake>(parent).is_none() {
                warn!("Added SnakeSegment is not a child of a Snake entity; self destructing now");
                world.commands().entity(ctx.entity).despawn();
            };

            // let sibling = world
            //     .get::<Children>(parent)
            //     .expect("We are a child so it must have Children");
            // if let Some(pre) = sibling.last()
            //     && let Some(pos) = world.get::<Transform>(*pre).copied()
            //     && let Some(mut seg_transform) = world.get_mut::<Transform>(ctx.entity)
            // {
            //     seg_transform.translation = pos.translation;
            // }
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

#[derive(Resource, Component, Deref, DerefMut)]
pub struct SnakeSize(pub f32);

impl Default for SnakeSize {
    fn default() -> Self {
        SnakeSize(SNAKE_SIZE)
    }
}
