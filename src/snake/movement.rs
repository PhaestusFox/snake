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
    snakes: Populated<(Option<&SnakeSize>, &Children), With<Snake>>,
    mut snake_segments: Query<&mut Transform>,
    fallback_size: Res<SnakeSize>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    let half_width = window.width() / 2.0;
    let half_height = window.height() / 2.0;

    for (size, segments) in &snakes {
        let size = **size.unwrap_or(&fallback_size);
        let w_step = (window.width() / size).ceil() * size;
        let h_step = (window.height() / size).ceil() * size;
        for child in segments.iter() {
            if let Ok(mut c_transform) = snake_segments.get_mut(child) {
                let mut delta = Vec3::ZERO;
                if c_transform.translation.x - (size / 2.) >= half_width {
                    delta.x -= w_step - 1.;
                } else if c_transform.translation.x + (size / 2.) <= -half_width {
                    delta.x += w_step - 1.;
                }

                if c_transform.translation.y - (size / 2.) >= half_height {
                    delta.y -= h_step - 1.;
                } else if c_transform.translation.y + (size / 2.) <= -half_height {
                    delta.y += h_step - 1.;
                }
                c_transform.translation += delta;
            }
        }
    }
}
