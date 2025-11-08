use bevy::window::PrimaryWindow;

use super::*;

pub fn plugin(app: &mut App) {
    app.add_systems(First, correct_local_snake_grid_to_global_grid);
    app.add_systems(FixedUpdate, (move_snake, wrap_screen).chain());
}

pub fn move_snake(
    mut snakes: Populated<(&SnakeSize, &Children, &FacingDirection), With<Snake>>,
    mut segments: Query<(&mut Transform, &mut FacingDirection), Without<Snake>>,
) {
    for (size, body, direction) in &mut snakes {
        let size = **size;
        let head = body[0];
        let Ok((_, mut head_facing)) = segments.get_mut(head) else {
            warn!("Failed to get head off Snake");
            continue;
        };
        *head_facing = *direction;
        for segment in body.windows(2).rev() {
            let Ok([(t, f), (mut m, mut r)]) = segments.get_many_mut([segment[0], segment[1]])
            else {
                warn!("Failed to get head segment facing direction");
                continue;
            };
            m.translation.x = t.translation.x;
            m.translation.y = t.translation.y;
            *r = *f;
        }
        let Ok((mut head_transform, _)) = segments.get_mut(head) else {
            warn!("Failed to get head off Snake");
            continue;
        };
        head_transform.translation += direction.move_vec() * size;
    }
}

fn wrap_screen(
    snakes: Query<(&Children, &SnakeSize), With<Snake>>,
    mut snake_segments: Query<&mut Transform, With<SnakeSegment>>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    let width = window.width();
    let height = window.height();
    // let width = crate::RESOLUTION.0 as f32;
    // let height = crate::RESOLUTION.1 as f32;
    let half_width = width / 2.;
    let half_height = height / 2.;

    for (body, size) in &snakes {
        let segment_size = **size;
        for segment in body {
            if let Ok(mut transform) = snake_segments.get_mut(*segment) {
                let translation = transform.translation;
                if translation.x > half_width + size.offset() {
                    transform.translation.x -= width + segment_size;
                } else if translation.x < -half_width - size.offset() {
                    transform.translation.x += width + segment_size;
                }

                if translation.y > half_height + size.offset() {
                    transform.translation.y -= height + segment_size;
                } else if translation.y < -half_height - size.offset() {
                    transform.translation.y += height + segment_size;
                }
            }
        }
    }
}

fn correct_local_snake_grid_to_global_grid(
    mut snakes: Populated<(&mut Transform, &SnakeSize), Changed<SnakeSize>>,
) {
    for (mut transform, size) in &mut snakes {
        transform.translation.x = size.offset();
        transform.translation.y = size.offset();
    }
}
