use bevy::window::PrimaryWindow;

use super::*;

pub fn plugin(app: &mut App) {
    app.add_systems(FixedUpdate, (move_snake, wrap_screen).chain());
}

pub fn move_snake(
    mut snakes: Populated<(Option<&SnakeSize>, &Children, &FacingDirection), With<Snake>>,
    mut segments: Query<(&mut Transform, &mut FacingDirection), Without<Snake>>,
    fallback_size: Res<SnakeSize>,
) {
    for (size, body, direction) in &mut snakes {
        let size = **size.unwrap_or(&fallback_size);
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
        head_transform.translation += direction.to_vec() * size;
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
