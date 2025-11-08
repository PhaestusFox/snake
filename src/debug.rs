use bevy::{asset::ron::de, ecs::relationship::Relationship};
use strum::IntoEnumIterator;

use crate::{
    map::{Map, ObjectSize, mini_map::MiniMapColor},
    snake::{PlayerSnake, Snake, SnakeSegment, SnakeSize},
};

use super::*;

pub struct TestPowerPlugin;

impl Plugin for TestPowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            First,
            (
                single_step,
                toggle_single_step,
                change_skin,
                change_snake_size,
            ),
        );

        app.add_systems(Startup, draw_debug_square);

        app.add_systems(OnEnter(DebugRenderMode::Colliders), spawn_collider_bounds)
            .add_systems(
                Update,
                add_new_colliders_on_spawning
                    .run_if(in_state(DebugRenderMode::Colliders))
                    .in_set(DebugOnlySystems),
            )
            .init_state::<DebugState>()
            .init_state::<DebugRenderMode>()
            .add_systems(Last, toggle_debug)
            .add_systems(Update, turn_on_colliders_render.in_set(DebugOnlySystems))
            .configure_sets(Update, DebugOnlySystems.run_if(in_state(DebugState::On)));
    }
}

fn single_step(world: &mut World) {
    if world
        .resource::<ButtonInput<KeyCode>>()
        .just_pressed(KeyCode::Space)
    {
        world.run_schedule(bevy::app::FixedMain);
    }
}

fn toggle_single_step(input: Res<ButtonInput<KeyCode>>, mut time: ResMut<Time<Virtual>>) {
    if input.just_pressed(KeyCode::F12) {
        if time.relative_speed() < 0.1 {
            time.set_relative_speed(1.0);
        } else {
            time.set_relative_speed(0.0);
        }
    }
}

fn change_skin(
    mut snakes: Query<&mut Snake, With<PlayerSnake>>,
    input: Res<ButtonInput<KeyCode>>,
    snake_types: Res<Assets<snake::SnakeType>>,
    asset_server: Res<AssetServer>,
) {
    if input.just_pressed(KeyCode::F1) {
        for mut snake_type in &mut snakes {
            let Some(skin) = snake_types.get(snake_type.as_ref()) else {
                warn!(
                    "Failed to get snake type {} for skin change",
                    snake_type.as_ref()
                );
                continue;
            };
            let next = skin.id.next();
            **snake_type = asset_server.load(next);
        }
    }
}

fn change_snake_size(
    input: Res<ButtonInput<KeyCode>>,
    // all snake with override size
    mut snakes: Query<(&Children, &mut SnakeSize), With<PlayerSnake>>,
    mut segments: Query<&mut Transform>,
) {
    let effect: fn(&mut SnakeSize);
    if input.just_pressed(KeyCode::PageUp) {
        effect = SnakeSize::inc;
    } else if input.just_pressed(KeyCode::PageDown) {
        effect = SnakeSize::dec;
    } else {
        return;
    };

    for (body, mut size) in &mut snakes {
        let old = *size;
        effect(&mut size);
        let new = *size;
        if new == old {
            continue;
        }

        let factor = *new / *old;

        let Ok(mut head_pos) = segments.get_mut(body[0]) else {
            warn!("Failed to get snake head transform");
            continue;
        };
        let origin = head_pos.translation;
        let new_origin = ((origin / *old) / factor).trunc() * *new;
        head_pos.translation = new_origin;

        // reposition all segments based on new size
        // skip head as it will be considered the origin
        for segment in body.iter().skip(1) {
            let Ok(mut seg_transform) = segments.get_mut(segment) else {
                warn!("Failed to get snake segment transform");
                continue;
            };
            let z = seg_transform.translation.z;
            let offset = (seg_transform.translation - origin) * factor;
            seg_transform.translation = new_origin + offset;
            seg_transform.translation.z = z;
        }
    }
}

fn draw_debug_square(mut commands: Commands, map: Res<Map>) {
    commands.spawn((
        Sprite {
            color: Color::WHITE,
            custom_size: Some(map.size().as_vec2() * GRID_SIZE),
            ..default()
        },
        Transform::from_translation(Vec3 {
            x: if map.size().x % 2 == 0 {
                -GRID_SIZE * 0.5
            } else {
                0.0
            },
            y: if map.size().y % 2 == 0 {
                -GRID_SIZE * 0.5
            } else {
                0.0
            },
            z: -100.0,
        }),
    ));
    for x in 0..map.size().x {
        for y in 0..map.size().y {
            if (x + y) % 2 == 0 {
                continue;
            }
            let x = x - map.size().x / 2;
            let y = y - map.size().y / 2;
            commands.spawn((
                Sprite {
                    color: Color::linear_rgb(0.0, 0.1, 0.0),
                    custom_size: Some(Vec2::splat(GRID_SIZE)),
                    ..default()
                },
                Transform::from_translation(Vec3::new(
                    x as f32 * GRID_SIZE,
                    y as f32 * GRID_SIZE,
                    -99.0,
                )),
            ));
        }
    }
}

fn spawn_collider_bounds(
    mut commands: Commands,
    object: Query<(Entity, &ObjectSize, Option<&MiniMapColor>)>,
    snakes: Query<(&Children, &SnakeSize, Option<&MiniMapColor>)>,
) {
    for (entity, size, color) in object.iter() {
        let color = color
            .map(|c| **c)
            .unwrap_or(Color::linear_rgba(1.0, 0.0, 1.0, 0.3))
            .with_alpha(0.3);
        commands.entity(entity).with_child((
            Sprite {
                custom_size: Some(Vec2::new(
                    size.0.x as f32 * GRID_SIZE,
                    size.0.y as f32 * GRID_SIZE,
                )),
                color,
                ..Default::default()
            },
            DespawnOnExit(DebugState::On),
            DebugRender::Colliders,
        ));
    }
    for (children, snake_size, color) in snakes.iter() {
        let size = **snake_size;
        let color = color
            .map(|c| **c)
            .unwrap_or(Color::linear_rgba(0.0, 1.0, 1.0, 0.3))
            .with_alpha(0.3);
        for segment in children.iter() {
            commands.entity(segment).with_child((
                Sprite {
                    custom_size: Some(Vec2::splat(size)),
                    color,
                    ..Default::default()
                },
                DespawnOnExit(DebugState::On),
                DebugRender::Colliders,
            ));
        }
    }
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
#[component(on_add = Self::dedupe)]
enum DebugRender {
    Colliders,
}
impl DebugRender {
    fn dedupe(mut world: bevy::ecs::world::DeferredWorld, ctx: bevy::ecs::lifecycle::HookContext) {
        // Find the parent entity
        let Some(parent) = world.get::<ChildOf>(ctx.entity).cloned() else {
            return;
        };
        // get all other children of the parent
        let Some(siblings) = world.get::<Children>(parent.get()) else {
            return;
        };
        let mode = *world.get::<DebugRender>(ctx.entity).expect("This is Self");
        for sibling in siblings.iter() {
            // skip self
            if sibling == ctx.entity {
                continue;
            }
            // check if sibling is the same debug object type
            if let Some(other_mode) = world.get::<DebugRender>(sibling)
                && mode.eq(other_mode)
            {
                // remove self since is duplicate
                world.commands().entity(ctx.entity).despawn();
                return;
            }
        }
    }
}

#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone)]
enum DebugState {
    #[default]
    Off,
    On,
}

#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone)]
enum DebugRenderMode {
    #[default]
    None,
    Colliders,
}

fn toggle_debug(
    mut debug_state: ResMut<NextState<DebugState>>,
    current: Res<State<DebugState>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Backquote) {
        if let DebugState::On = current.get() {
            debug_state.set(DebugState::Off);
        } else {
            debug_state.set(DebugState::On);
        }
    }
}

fn turn_on_colliders_render(
    mut debug_state: ResMut<NextState<DebugRenderMode>>,
    mut debug_render_mode: ResMut<State<DebugRenderMode>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::F9) {
        *debug_render_mode = State::new(DebugRenderMode::None);
        debug_state.set(DebugRenderMode::Colliders);
    }
}

fn add_new_colliders_on_spawning(
    mut commands: Commands,
    new_objects: Query<(Entity, &ObjectSize, Option<&MiniMapColor>), Added<ObjectSize>>,
    snakes: Query<(&SnakeSize, Option<&MiniMapColor>)>,
    new_snakes: Query<(Entity, &ChildOf), Added<SnakeSegment>>,
) {
    for (entity, size, color) in new_objects.iter() {
        commands.entity(entity).with_child((
            Sprite {
                custom_size: Some(size.size()),
                color: color
                    .map(|v| **v)
                    .unwrap_or(Color::linear_rgba(1.0, 0.0, 1.0, 0.3))
                    .with_alpha(0.3),
                ..Default::default()
            },
            DespawnOnExit(DebugState::On),
            DebugRender::Colliders,
        ));
    }
    for (segment, snake) in new_snakes.iter() {
        let Ok((size, color)) = snakes.get(snake.get()) else {
            warn!("Failed to get snake size for new segment debug collider");
            continue;
        };
        let size = **size;
        commands.entity(segment).with_child((
            Sprite {
                custom_size: Some(Vec2::splat(size)),
                color: color
                    .map(|v| **v)
                    .unwrap_or(Color::linear_rgba(1.0, 0.0, 1.0, 0.3))
                    .with_alpha(0.3),
                ..Default::default()
            },
            DespawnOnExit(DebugState::On),
            DebugRender::Colliders,
        ));
    }
}

#[derive(SystemSet, Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct DebugOnlySystems;
