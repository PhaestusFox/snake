use bevy::window::PrimaryWindow;

use super::*;

pub fn plugin(app: &mut App) {
    app.add_systems(
        FixedUpdate,
        (move_snake, (update_path, wrap_screen)).chain(),
    );
}

fn move_snake(
    mut snakes: Populated<
        (
            &mut Transform,
            &FacingDirection,
            Option<&SnakeSize>,
            Option<&Children>,
        ),
        With<Snake>,
    >,
    mut segments: Query<(&mut Transform, &FacingDirection), (Without<Snake>, With<SnakeSegment>)>,
    fallback_size: Res<SnakeSize>,
) {
    for (mut transform, direction, size, children) in &mut snakes {
        let size = size.unwrap_or(&fallback_size);
        let delta = direction.to_vec() * **size;
        transform.translation += delta;
        if let Some(body) = children {
            // we need to skip the head so its always at Vec3::ZERO relative to the snake
            let mut head = true;
            for segment in body.iter() {
                if let Ok((mut seg_transform, seg_direction)) = segments.get_mut(segment) {
                    // the head is the first SnakeSegment
                    // we can't just skip the first child because it could be something other then a SnakeSegment like a NameTag
                    if head {
                        head = false;
                        continue;
                    }
                    seg_transform.translation += seg_direction.to_vec() * **size - delta;
                }
            }
        }
    }
}

fn update_path(
    snakes: Query<(Entity, &Children), With<Snake>>,
    mut facing: Query<&mut FacingDirection>,
) {
    for (head, segments) in snakes.iter() {
        let Ok(mut last) = facing.get(head).cloned() else {
            continue;
        };

        // we need to skip the head
        let mut head = true;
        for segment in segments.iter() {
            if let Ok(mut dir) = facing.get_mut(segment) {
                if head {
                    // the head needs the direction to set its image correctly
                    // but we can't swap or we end up with a break in the snake
                    head = false;
                    *dir = last;
                    continue;
                }
                std::mem::swap(&mut last, &mut dir);
            }
        }
    }
}

fn wrap_screen(
    mut snakes: Populated<(&mut Transform, Option<&SnakeSize>, &Children), With<Snake>>,
    mut snake_segments: Populated<
        (&mut Transform, Option<&SnakeSize>),
        (With<SnakeSegment>, Without<Snake>),
    >,
    fallback_size: Res<SnakeSize>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    let half_width = window.width() / 2.0;
    let half_height = window.height() / 2.0;

    for (mut transform, size, segments) in &mut snakes {
        let size = **size.unwrap_or(&fallback_size);
        let w_in_steps = (half_width / size).round();
        let h_in_steps = (half_height / size).round();
        let mut delta = Vec3::ZERO;
        if transform.translation.x > half_width + size {
            delta.x -= (w_in_steps + 1.) * 2. * size;
        } else if transform.translation.x < -half_width - size {
            delta.x += (w_in_steps + 1.) * 2. * size;
        }
        if transform.translation.y > half_height + size {
            delta.y -= (h_in_steps + 1.) * 2. * size;
        } else if transform.translation.y < -half_height - size {
            delta.y += (h_in_steps + 1.) * 2. * size;
        }
        transform.translation += delta;
        for child in segments.iter() {
            if let Ok((mut seg_transform, _)) = snake_segments.get_mut(child) {
                seg_transform.translation -= delta;
            }
        }
    }

    for (mut transform, size) in &mut snake_segments {
        let size = **size.unwrap_or(&fallback_size);
        let w_in_steps = (half_width / size).round();
        let h_in_steps = (half_height / size).round();
        if transform.translation.x > half_width + size {
            transform.translation.x -= (w_in_steps + 1.) * 2. * size;
        } else if transform.translation.x < -half_width - size {
            transform.translation.x += (w_in_steps + 1.) * 2. * size;
        }

        if transform.translation.y > half_height + size {
            transform.translation.y -= (h_in_steps + 1.) * 2. * size;
        } else if transform.translation.y < -half_height - size {
            transform.translation.y += (h_in_steps + 1.) * 2. * size;
        }
    }
}
