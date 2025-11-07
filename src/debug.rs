use crate::snake::SnakeSize;

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
        world.run_schedule(FixedMain);
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

fn change_skin(mut snakes: Query<&mut snake::SnakeType>, input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::F1) {
        for mut snake_type in &mut snakes {
            *snake_type = snake_type.next();
        }
    }
}

fn change_snake_size(
    mut snake_size: ResMut<snake::SnakeSize>,
    input: Res<ButtonInput<KeyCode>>,
    // all snake with override size
    mut snakes: Query<(&Children, Option<&mut SnakeSize>)>,
    mut segments: Query<&mut Transform>,
) {
    let next: SnakeSize;
    let effect: fn(&mut SnakeSize);
    if input.just_pressed(KeyCode::PageUp) {
        next = snake_size.up();
        effect = SnakeSize::inc;
    } else if input.just_pressed(KeyCode::PageDown) {
        next = snake_size.down();
        effect = SnakeSize::dec;
    } else {
        return;
    };

    for (body, size) in &mut snakes {
        let old = *size.as_deref().unwrap_or(snake_size.as_ref());
        let new = size
            .map(|mut c| {
                effect(&mut c);
                *c
            })
            .unwrap_or(next);
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
    *snake_size = next;
}
