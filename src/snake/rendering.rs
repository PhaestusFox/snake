use bevy::{asset::LoadedFolder, platform::collections::HashMap, prelude::*};
use strum::IntoEnumIterator;

use super::SnakeType;
use crate::snake::{FacingDirection, Snake, SnakeSegment, SnakeSize};

pub struct SnakeRenderPlugin;

impl Plugin for SnakeRenderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SnakeTextureHandles>()
            .add_systems(FixedPostUpdate, update_snake_texture)
            .add_systems(
                FixedUpdate,
                SnakeTextureHandles::shuttle_finished_to_loaded
                    .run_if(SnakeTextureHandles::should_load),
            )
            .add_systems(Update, update_snake_size);
    }
}

impl SnakeType {
    pub fn segment_indices(&self) -> &'static [usize; 4] {
        match self {
            SnakeType::WhiteSpotted => &[2, 1, 0, 3],
            SnakeType::BlueArrow => &[58, 57, 56, 59],
        }
    }
}

fn update_snake_texture(
    snakes: Query<(&Snake, &SnakeType)>,
    mut segments: Populated<(&mut Sprite, &mut Transform, &FacingDirection)>,
    textures: Res<SnakeTextureHandles>,
) {
    for (snake, snake_type) in &snakes {
        let Some(textures) = textures.loaded.get(snake_type) else {
            warn!("{:?} textures not loaded yet", snake_type);
            continue;
        };
        if let Some(head) = snake.first() {
            if let Ok((mut sprite, mut pos, direction)) = segments.get_mut(*head) {
                sprite.image = textures[0].clone();
                match direction {
                    FacingDirection::Right => {
                        pos.rotation = Quat::from_rotation_z(0.);
                        sprite.flip_x = false;
                        sprite.flip_y = false;
                    }
                    FacingDirection::Down => {
                        pos.rotation = Quat::from_rotation_z(std::f32::consts::FRAC_PI_2);
                        sprite.flip_x = true;
                        sprite.flip_y = true;
                    }
                    FacingDirection::Left => {
                        pos.rotation = Quat::from_rotation_z(0.);
                        sprite.flip_x = true;
                        sprite.flip_y = false;
                    }
                    FacingDirection::Up => {
                        pos.rotation = Quat::from_rotation_z(std::f32::consts::FRAC_PI_2);
                        sprite.flip_x = false;
                        sprite.flip_y = true;
                    }
                    FacingDirection::None => {}
                }
            } else {
                warn!("Failed to get head segment sprite");
            }
        }
        for window in snake.windows(3) {
            let Ok([(_, pre, _), (mut s, mut main, _), (_, next, _)]) =
                segments.get_many_mut([window[0], window[1], window[2]])
            else {
                warn!("Failed to get middle segment sprite");
                continue;
            };
            let head = FacingDirection::moving(pre.translation, main.translation);
            let tail = FacingDirection::moving(next.translation, main.translation);
            let connection = Connection::new(head, tail);
            if connection.straight {
                s.image = textures[1].clone();
            } else {
                s.image = textures[2].clone();
            }
            main.rotation = Quat::from_rotation_z(connection.rotation);
            s.flip_y = connection.flip_y;
            s.flip_x = connection.flip_x;
        }
        if snake.len() > 1
            && let Some(tail) = snake.last()
            && let Some(second_last) = snake.get(snake.len() - 2)
        {
            if let Ok([(mut sprite, mut pos, _), (_, other, _)]) =
                segments.get_many_mut([*tail, *second_last])
            {
                let tail = FacingDirection::moving(pos.translation, other.translation);
                pos.rotation = Quat::from_rotation_z(tail.to_rotation());
                sprite.flip_x = false;
                sprite.flip_y = false;
                sprite.image = textures[3].clone();
            } else {
                warn!("Failed to get tail segment sprite");
            }
        }
    }
}

fn update_snake_size(
    mut segments: Query<(&mut Sprite, &SnakeSize), Changed<SnakeSize>>,
    mut fallback_segments: Query<&mut Sprite, (Without<SnakeSize>, With<SnakeSegment>)>,
    fallback_size: Res<SnakeSize>,
) {
    for (mut sprite, size) in &mut segments {
        sprite.custom_size = Some(Vec2::splat(**size));
    }
    if fallback_size.is_changed() {
        for mut sprite in &mut fallback_segments {
            sprite.custom_size = Some(Vec2::splat(**fallback_size));
        }
    }
}

#[derive(Resource)]
struct SnakeTextureHandles {
    loading: Option<Handle<LoadedFolder>>,
    loaded: HashMap<SnakeType, [Handle<Image>; 4]>,
}

impl SnakeTextureHandles {
    fn should_load(textures: Res<Self>, asset_server: Res<AssetServer>) -> bool {
        if let Some(loading) = &textures.loading {
            return asset_server.is_loaded(loading.id());
        }
        false
    }
    fn shuttle_finished_to_loaded(
        mut textures: ResMut<SnakeTextureHandles>,
        folders: Res<Assets<LoadedFolder>>,
    ) {
        let Some(loading) = textures.loading.take() else {
            return;
        };
        let Some(folder) = folders.get(loading.id()) else {
            panic!("Expected LoadedFolder to be present");
        };
        for snake_type in SnakeType::iter() {
            let indices = snake_type
                .segment_indices()
                .map(|id| folder.handles[id].clone().typed::<Image>());
            textures.loaded.insert(snake_type, indices);
        }
    }
}

impl FromWorld for SnakeTextureHandles {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        let texture_handles = asset_server.load_folder("no_share/BattleSnake/snakes/32x32px_split");
        SnakeTextureHandles {
            loading: Some(texture_handles),
            loaded: HashMap::new(),
        }
    }
}

struct Connection {
    flip_x: bool,
    flip_y: bool,
    straight: bool,
    rotation: f32,
}

impl Connection {
    fn new(next: FacingDirection, prev: FacingDirection) -> Self {
        match (next, prev) {
            (FacingDirection::Right, FacingDirection::Left) => Connection {
                flip_x: false,
                flip_y: false,
                straight: true,
                rotation: 0.0,
            },
            (FacingDirection::Left, FacingDirection::Right) => Connection {
                flip_x: true,
                flip_y: true,
                straight: true,
                rotation: 0.0,
            },
            (FacingDirection::Up, FacingDirection::Down) => Connection {
                flip_x: true,
                flip_y: false,
                straight: true,
                rotation: -std::f32::consts::FRAC_PI_2,
            },
            (FacingDirection::Down, FacingDirection::Up) => Connection {
                flip_x: false,
                flip_y: false,
                straight: true,
                rotation: -std::f32::consts::FRAC_PI_2,
            },
            (FacingDirection::Right, FacingDirection::Down) => Connection {
                flip_x: false,
                flip_y: false,
                straight: false,
                rotation: 0.0,
            },
            (FacingDirection::Right, FacingDirection::Up) => Connection {
                flip_x: false,
                flip_y: true,
                straight: false,
                rotation: 0.0,
            },
            (FacingDirection::Left, FacingDirection::Down) => Connection {
                flip_x: true,
                flip_y: false,
                straight: false,
                rotation: 0.0,
            },
            (FacingDirection::Left, FacingDirection::Up) => Connection {
                flip_x: true,
                flip_y: true,
                straight: false,
                rotation: 0.0,
            },
            (FacingDirection::Down, FacingDirection::Right) => Connection {
                flip_x: true,
                flip_y: false,
                straight: false,
                rotation: std::f32::consts::FRAC_PI_2,
            },
            (FacingDirection::Down, FacingDirection::Left) => Connection {
                flip_x: false,
                flip_y: false,
                straight: false,
                rotation: -std::f32::consts::FRAC_PI_2,
            },
            (FacingDirection::Up, FacingDirection::Right) => Connection {
                flip_x: false,
                flip_y: false,
                straight: false,
                rotation: std::f32::consts::FRAC_PI_2,
            },
            (FacingDirection::Up, FacingDirection::Left) => Connection {
                flip_x: true,
                flip_y: false,
                straight: false,
                rotation: -std::f32::consts::FRAC_PI_2,
            },
            _ => {
                warn!("Not implemented: {next:?} {prev:?}");
                Connection {
                    flip_x: false,
                    flip_y: false,
                    straight: true,
                    rotation: 0.0,
                }
            }
        }
    }
}
