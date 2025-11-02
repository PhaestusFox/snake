use bevy::{
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    prelude::*,
};

mod rendering;

const SNAKE_SIZE: f32 = 32.0;

#[derive(Component, Deref, DerefMut, Default)]
#[require(Transform, SnakeType, Visibility)]
pub struct Snake(Vec<Entity>);

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(rendering::SnakeRenderPlugin)
            .add_systems(FixedUpdate, (move_snake, update_path).chain())
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
    pub fn moving(front: Vec3, back: Vec3) -> Self {
        let dir = front - back;
        if dir.x.abs() > dir.y.abs() {
            if dir.x > 0.0 {
                FacingDirection::Right
            } else {
                FacingDirection::Left
            }
        } else if dir.y > 0.0 {
            FacingDirection::Up
        } else {
            FacingDirection::Down
        }
    }

    pub fn to_rotation(self) -> f32 {
        match self {
            FacingDirection::Left => 0.0,
            FacingDirection::Up => -std::f32::consts::FRAC_PI_2,
            FacingDirection::Right => std::f32::consts::PI,
            FacingDirection::Down => std::f32::consts::FRAC_PI_2,
            FacingDirection::None => 0.0,
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

fn move_snake(
    mut segment: Populated<(&mut Transform, &FacingDirection, Option<&SnakeSize>)>,
    fallback_size: Res<SnakeSize>,
) {
    for (mut transform, direction, size) in &mut segment {
        let size = size.unwrap_or(&fallback_size);
        match direction {
            FacingDirection::Right => transform.translation.x += **size,
            FacingDirection::Down => transform.translation.y -= **size,
            FacingDirection::Left => transform.translation.x -= **size,
            FacingDirection::Up => transform.translation.y += **size,
            FacingDirection::None => {}
        }
    }
}

fn update_path(snakes: Query<&Snake>, mut facing: Query<&mut FacingDirection>) {
    for snake in snakes.iter() {
        for segments in snake.windows(2).rev() {
            if let Ok([mut dir, frount]) = facing.get_many_mut([segments[1], segments[0]]) {
                *dir = *frount;
            }
        }
    }
}

fn user_input(
    snakes: Query<&Snake>,
    mut facing: Query<&mut FacingDirection>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for snake in snakes.iter() {
        if let Ok(mut dir) = facing.get_mut(snake[0]) {
            if keys.pressed(KeyCode::KeyD) {
                *dir = FacingDirection::Right;
            } else if keys.pressed(KeyCode::KeyS) {
                *dir = FacingDirection::Down;
            } else if keys.pressed(KeyCode::KeyA) {
                *dir = FacingDirection::Left;
            } else if keys.pressed(KeyCode::KeyW) {
                *dir = FacingDirection::Up;
            }
        }
    }
}

#[derive(Component)]
#[component(on_add = Self::on_add)]
#[require(Transform, FacingDirection, Sprite)]
pub struct SnakeSegment;

impl SnakeSegment {
    fn on_add(mut world: DeferredWorld, ctx: HookContext) {
        if let Some(&ChildOf(parent)) = world.get::<ChildOf>(ctx.entity) {
            if let Some(mut snake) = world.get_mut::<Snake>(parent) {
                let pre = snake.last().cloned();
                snake.push(ctx.entity);
                if let Some(pre) = pre
                    && let Some(pos) = world.get::<Transform>(pre).copied()
                    && let Some(mut seg_transform) = world.get_mut::<Transform>(ctx.entity)
                {
                    seg_transform.translation = pos.translation;
                }
            }
            if let Some(snake_type) = world.get::<SnakeType>(parent).copied()
                && let Some(mut seg_type) = world.get_mut::<SnakeType>(ctx.entity)
            {
                *seg_type = snake_type;
            }
        } else {
            warn!("Added SnakeSegment is not a child of a Snake entity");
        };
    }
}

#[derive(Clone, Copy, Component, Default, PartialEq, Eq, Hash, strum_macros::EnumIter, Debug)]
pub enum SnakeType {
    #[default]
    WhiteSpotted = 0,
    BlueArrow = 14,
}

#[derive(Resource, Component, Deref, DerefMut)]
pub struct SnakeSize(pub f32);

impl Default for SnakeSize {
    fn default() -> Self {
        SnakeSize(SNAKE_SIZE)
    }
}
