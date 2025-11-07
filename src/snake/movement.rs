use bevy::window::PrimaryWindow;

use super::*;

pub fn plugin(app: &mut App) {
    app.add_systems(
        FixedUpdate,
        (
            load_user_input_into_head,
            move_snake,
            (update_path, wrap_screen),
        )
            .chain(),
    );
}

fn load_user_input_into_head(
    snake: Query<(&FacingDirection, &Children), With<Snake>>,
    mut segments: Query<&mut FacingDirection, Without<Snake>>,
) {
    for (direction, body) in snake.iter() {
        let Some(head) = body.first() else {
            warn!("Snake has no segments");
            continue;
        };
        if let Ok(mut head_direction) = segments.get_mut(*head)
            && head_direction.ne(&direction.invers())
        {
            *head_direction = *direction;
        } else {
            warn!("Failed to get head segment facing direction");
        }
    }
}

fn move_snake(
    mut snakes: Populated<(Option<&SnakeSize>, &Children), With<Snake>>,
    mut segments: Query<(&mut Transform, &FacingDirection), Without<Snake>>,
    fallback_size: Res<SnakeSize>,
) {
    for (size, body) in &mut snakes {
        let size = **size.unwrap_or(&fallback_size);
        for segment in body.iter() {
            let Ok((mut seg_transform, direction)) = segments.get_mut(segment) else {
                warn!("Failed to get snake segment transform");
                continue;
            };
            seg_transform.translation += direction.to_vec() * size;
        }
    }
}

fn update_path(snakes: Query<&Children, With<Snake>>, mut facing: Query<&mut FacingDirection>) {
    for segments in snakes.iter() {
        for segment in segments.windows(2).rev() {
            let Ok([f, mut s]) = facing.get_many_mut([segment[0], segment[1]]) else {
                warn!("Failed to get head segment facing direction");
                continue;
            };
            *s = *f;
        }
    }
}

fn wrap_screen(
    snakes: Query<(&Children, Option<&SnakeSize>), With<Snake>>,
    mut snake_segments: Query<&mut Transform, With<SnakeSegment>>,
    fallback_size: Res<SnakeSize>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    let width = window.width();
    let height = window.height();
    // let width = crate::RESOLUTION.0 as f32;
    // let height = crate::RESOLUTION.1 as f32;
    let half_width = width / 2.;
    let half_height = height / 2.;

    for (body, size) in &snakes {
        let size = **size.unwrap_or(&fallback_size);
        for segment in body {
            if let Ok(mut transform) = snake_segments.get_mut(*segment) {
                if transform.translation.x > half_width {
                    transform.translation.x -= width + size;
                } else if transform.translation.x < -half_width {
                    transform.translation.x += width + size;
                }

                if transform.translation.y > half_height {
                    transform.translation.y -= height + size;
                } else if transform.translation.y < -half_height {
                    transform.translation.y += height + size;
                }
            }
        }
    }
}
