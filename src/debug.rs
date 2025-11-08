use crate::snake::{PlayerSnake, Snake, SnakeSize};

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
    let next: SnakeSize;
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
