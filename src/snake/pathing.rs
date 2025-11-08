use super::*;

pub fn plugin(app: &mut App) {
    app.add_systems(FixedPreUpdate, apply_path);
    app.add_systems(FixedFirst, apply_path_finding);
    #[cfg(debug_assertions)]
    {
        app.add_systems(FixedLast, record_path);
    }
}

fn apply_path(mut snakes: Populated<(&mut FacingDirection, &mut SnakePath), With<Snake>>) {
    for (mut facing, mut pathing) in &mut snakes {
        let Some(next) = pathing.pop() else {
            continue;
        };
        *facing = next;
    }
}

#[derive(Component, Deref, DerefMut, Default)]
struct SnakePath(Vec<FacingDirection>);

#[derive(Component)]
#[require(SnakePath)]
pub enum PathFinding {
    FixedPath(Vec<FacingDirection>),
}

fn apply_path_finding(mut snakes: Populated<(&mut SnakePath, &PathFinding), Without<PlayerSnake>>) {
    for (mut current, path_finding) in &mut snakes {
        if let Err(e) = path_finding.compute_path(current.as_mut()) {
            warn!("Failed to compute path for snake: {e:?}");
        }
    }
}

impl PathFinding {
    fn compute_path(&self, current: &mut SnakePath) -> Result<(), ()> {
        match self {
            PathFinding::FixedPath(path) => {
                if current.len() < 2 {
                    current.extend(path.iter().cloned());
                }
            }
        }
        Ok(())
    }
}

fn record_path(
    player: Single<(&FacingDirection, &Snake, Option<&SnakeSize>, &Children), With<PlayerSnake>>,
    segment: Query<&Transform>,
    mut new: Local<Option<(Vec<FacingDirection>, Vec3)>>,
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
) {
    if input.pressed(KeyCode::F7) {
        let Ok(segment) = segment.get(player.3[0]) else {
            warn!("Player Snake has no head segment?");
            return;
        };
        *new = Some((vec![*player.0], segment.translation));
        println!("Started recording path");
        return;
    }
    let Some(path) = new.as_mut() else {
        return;
    };
    let mut len = 0;
    for key in input.get_pressed() {
        match key {
            KeyCode::Numpad0 => {
                *new = None;
                return;
            }
            KeyCode::Numpad1 => {
                len = 1;
                break;
            }
            KeyCode::Numpad2 => {
                len = 2;
                break;
            }
            KeyCode::Numpad3 => {
                len = 3;
                break;
            }
            KeyCode::Numpad4 => {
                len = 4;
                break;
            }
            KeyCode::Numpad5 => {
                len = 5;
                break;
            }
            KeyCode::Numpad6 => {
                len = 6;
                break;
            }
            KeyCode::Numpad7 => {
                len = 7;
                break;
            }
            KeyCode::Numpad8 => {
                len = 8;
                break;
            }
            KeyCode::Numpad9 => {
                len = 9;
                break;
            }
            _ => {
                continue;
            }
        }
    }
    if len == 0 {
        path.0.push(*player.0);
    } else {
        let Some((path, start)) = new.take() else {
            warn!("No recorded path to spawn snake from");
            return;
        };
        let mut new_snake = commands.spawn((player.1.clone(), PathFinding::FixedPath(path)));
        if let Some(size) = player.2 {
            new_snake.insert(*size);
        }
        for _ in 0..len {
            new_snake.with_child((SnakeSegment, Transform::from_translation(start)));
        }
    }
}
