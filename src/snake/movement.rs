use bevy::window::PrimaryWindow;

use super::*;

pub fn plugin(app: &mut App) {
    app.add_systems(
        FixedUpdate,
        (move_snake, (update_path, wrap_screen)).chain(),
    );
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

fn wrap_screen(
    mut snake_segments: Populated<(&mut Transform, Option<&SnakeSize>), With<SnakeSegment>>,
    fallback_size: Res<SnakeSize>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    let half_width = window.width() / 2.0;
    let half_height = window.height() / 2.0;

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
