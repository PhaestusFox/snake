use bevy::asset::RenderAssetUsages;

use crate::{
    collectables::Collectable,
    snake::{PlayerSnake, SnakeSize},
};

use super::*;

pub struct MiniMapPlugin;

impl Plugin for MiniMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_mini_map);
        app.add_systems(FixedUpdate, clear_mini_map);
        app.add_systems(
            FixedPostUpdate,
            (
                draw_other_snakes_on_mini_map,
                draw_collectable_on_mini_map,
                draw_player_on_mini_map,
            )
                .chain(),
        );
    }
}

#[derive(Component)]
#[require(ImageNode)]
pub struct MiniMap;

#[derive(Component, Deref)]
pub struct MiniMapColor(pub Color);

fn spawn_mini_map(mut commands: Commands, mut images: ResMut<Assets<Image>>, map: Res<Map>) {
    let map_image = Image::new_fill(
        bevy::render::render_resource::Extent3d {
            width: map.size.x,
            height: map.size.y,
            depth_or_array_layers: 1,
        },
        bevy::render::render_resource::TextureDimension::D2,
        &[0, 0, 0, 255],
        bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::all(),
    );

    let map_image_handle = images.add(map_image);
    commands.spawn((
        MiniMap,
        Node {
            top: Val::Px(10.0),
            right: Val::Px(10.0),
            position_type: PositionType::Absolute,
            width: Val::VMin(25.),
            height: Val::VMin(25.),
            min_height: Val::Px(map.size().y as f32),
            min_width: Val::Px(map.size().x as f32),
            ..Default::default()
        },
        ImageNode {
            image: map_image_handle,
            ..Default::default()
        },
    ));
}

fn clear_mini_map(mut images: ResMut<Assets<Image>>, mini_map: Single<&ImageNode, With<MiniMap>>) {
    let Some(mini_map_image) = images.get_mut(&mini_map.image) else {
        warn!("Failed to get mini map image to clear");
        return;
    };
    for x in 0..mini_map_image.width() {
        for y in 0..mini_map_image.height() {
            _ = mini_map_image.set_color_at(x, y, Color::BLACK);
        }
    }
}

fn draw_player_on_mini_map(
    mut images: ResMut<Assets<Image>>,
    player_snake: Single<(&Children, &MiniMapColor, Option<&SnakeSize>), With<PlayerSnake>>,
    fallback_size: Res<SnakeSize>,
    mini_map: Single<&ImageNode, With<MiniMap>>,
    segments: Query<&Transform>,
    map: Res<Map>,
) {
    let Some(mini_map_image) = images.get_mut(&mini_map.image) else {
        warn!("Failed to get mini map image to draw player");
        return;
    };
    let (player_children, MiniMapColor(color), snake_size) = player_snake.into_inner();
    let size = **snake_size.unwrap_or(&fallback_size);
    let size_in_cells = (size / WORLD_GRID_SIZE) as u32;
    for segment in player_children.iter() {
        let Ok(segment_transform) = segments.get(segment) else {
            warn!("Failed to get player snake segment transform for mini map");
            continue;
        };
        let mx =
            (segment_transform.translation.x / WORLD_GRID_SIZE.trunc()) as i32 + (map.size().x / 2);
        let my =
            (map.size().y / 2) - (segment_transform.translation.y / WORLD_GRID_SIZE.trunc()) as i32;
        if mx < 0
            || my < 0
            || mx as u32 >= mini_map_image.width()
            || my as u32 >= mini_map_image.height()
        {
            continue;
        }
        for dx in 0..size_in_cells {
            for dy in 0..size_in_cells {
                let x = (mx as u32).saturating_add(dx);
                let y = (my as u32).saturating_add(dy);
                if x >= mini_map_image.width() || y >= mini_map_image.height() {
                    continue;
                }
                _ = mini_map_image.set_color_at(x, y, *color);
            }
        }
    }
}

fn draw_collectable_on_mini_map(
    mut images: ResMut<Assets<Image>>,
    collectables: Query<(&Transform, &MiniMapColor, &ObjectSize), With<Collectable>>,
    mini_map: Single<&ImageNode, With<MiniMap>>,
    map: Res<Map>,
) {
    let Some(mini_map_image) = images.get_mut(&mini_map.image) else {
        warn!("Failed to get mini map image to draw collectables");
        return;
    };
    for (transform, MiniMapColor(color), size) in collectables.iter() {
        let mx = (transform.translation.x / WORLD_GRID_SIZE.trunc()) as i32 + (map.size().x / 2);
        let my = (map.size().y / 2) - (transform.translation.y / WORLD_GRID_SIZE.trunc()) as i32;
        if mx < 0
            || my < 0
            || mx as u32 >= mini_map_image.width()
            || my as u32 >= mini_map_image.height()
        {
            continue;
        }
        for dx in 0..size.0.x {
            for dy in 0..size.0.y {
                let x = (mx as u32).saturating_add(dx);
                let y = (my as u32).saturating_add(dy);
                if x >= mini_map_image.width() || y >= mini_map_image.height() {
                    continue;
                }
                _ = mini_map_image.set_color_at(x, y, *color);
            }
        }
    }
}

fn draw_other_snakes_on_mini_map(
    mut images: ResMut<Assets<Image>>,
    snakes: Query<(&Children, Option<&MiniMapColor>, Option<&SnakeSize>), Without<PlayerSnake>>,
    fallback_size: Res<SnakeSize>,
    mini_map: Single<&ImageNode, With<MiniMap>>,
    segments: Query<&Transform>,
    map: Res<Map>,
) {
    let Some(mini_map_image) = images.get_mut(&mini_map.image) else {
        warn!("Failed to get mini map image to draw player");
        return;
    };
    for (player_children, mini_map_color, snake_size) in snakes.iter() {
        let size = **snake_size.unwrap_or(&fallback_size);
        let color = **mini_map_color.unwrap_or(&MiniMapColor(Color::linear_rgb(1., 0.0, 0.0)));
        let size_in_cells = (size / WORLD_GRID_SIZE) as u32;
        for segment in player_children.iter() {
            let Ok(segment_transform) = segments.get(segment) else {
                warn!("Failed to get player snake segment transform for mini map");
                continue;
            };
            let mx = (segment_transform.translation.x / WORLD_GRID_SIZE.trunc()) as i32
                + (map.size().x / 2);
            let my = (map.size().y / 2)
                - (segment_transform.translation.y / WORLD_GRID_SIZE.trunc()) as i32;
            if mx < 0
                || my < 0
                || mx as u32 >= mini_map_image.width()
                || my as u32 >= mini_map_image.height()
            {
                continue;
            }
            for dx in 0..size_in_cells {
                for dy in 0..size_in_cells {
                    let x = (mx as u32).saturating_add(dx);
                    let y = (my as u32).saturating_add(dy);
                    if x >= mini_map_image.width() || y >= mini_map_image.height() {
                        continue;
                    }
                    _ = mini_map_image.set_color_at(x, y, color);
                }
            }
        }
    }
}
