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
            min_height: Val::Px(map.size().y as f32 * 20.),
            min_width: Val::Px(map.size().x as f32 * 20.),
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
    player_snake: Single<(&Children, &MiniMapColor, &SnakeSize), With<PlayerSnake>>,
    mini_map: Single<&ImageNode, With<MiniMap>>,
    segments: Query<&Transform>,
    map: Res<Map>,
) {
    let Some(mini_map_image) = images.get_mut(&mini_map.image) else {
        warn!("Failed to get mini map image to draw player");
        return;
    };
    let (player_children, MiniMapColor(color), snake_size) = player_snake.into_inner();
    draw_snake_on_map(
        mini_map_image,
        &segments,
        player_children,
        *snake_size,
        *color,
        map.size(),
    );
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
        draw_object_on_mini_map(
            mini_map_image,
            transform.translation,
            *size,
            *color,
            map.size(),
        );
    }
}

fn draw_other_snakes_on_mini_map(
    mut images: ResMut<Assets<Image>>,
    snakes: Query<(&Children, Option<&MiniMapColor>, &SnakeSize), Without<PlayerSnake>>,
    mini_map: Single<&ImageNode, With<MiniMap>>,
    segments: Query<&Transform>,
    map: Res<Map>,
) {
    let Some(mini_map_image) = images.get_mut(&mini_map.image) else {
        warn!("Failed to get mini map image to draw player");
        return;
    };
    for (snake, mini_map_color, snake_size) in snakes.iter() {
        draw_snake_on_map(
            mini_map_image,
            &segments,
            snake,
            *snake_size,
            mini_map_color.map_or(Color::linear_rgb(1., 0.0, 0.0), |c| c.0),
            map.size(),
        );
    }
}

fn draw_snake_on_map(
    image: &mut Image,
    segments: &Query<&Transform>,
    snake: &Children,
    size: SnakeSize,
    color: Color,
    map_size: IVec2,
) {
    let size = ObjectSize(UVec2::splat(size.stride()));
    for segment in snake.iter() {
        let Ok(segment_transform) = segments.get(segment) else {
            warn!("Failed to get player snake segment transform for mini map");
            continue;
        };
        let translation = segment_transform.translation + size.offset().extend(0.);
        let start = calculate_mini_map_start(translation, size, map_size);
        fill_in_mini_map(image, start, size, color);
    }
}

fn draw_object_on_mini_map(
    image: &mut Image,
    center: Vec3,
    size: ObjectSize,
    color: Color,
    offset: IVec2,
) {
    let start = calculate_mini_map_start(center, size, offset);
    fill_in_mini_map(image, start, size, color);
}

fn calculate_mini_map_start(center: Vec3, size: ObjectSize, map_size: IVec2) -> IVec2 {
    let mx = (center.x / GRID_SIZE).floor() as i32 + map_size.x / 2;
    let mut my = (map_size.y / 2) - (center.y / GRID_SIZE).floor() as i32;
    if size.y.is_multiple_of(2) {
        my -= 1;
    }
    IVec2 { x: mx, y: my }
}

fn fill_in_mini_map(image: &mut Image, start: IVec2, size: ObjectSize, color: Color) {
    let rx = (size.x / 2) as i32;
    let ry = (size.y / 2) as i32;
    let ry = if size.y.is_multiple_of(2) {
        -ry + 1..=ry
    } else {
        -ry..=ry
    };
    let rx = if size.x.is_multiple_of(2) {
        -rx + 1..=rx
    } else {
        -rx..=rx
    };
    for dx in rx {
        for dy in ry.clone() {
            let x = start.x.saturating_add(dx) as u32;
            let y = start.y.saturating_add(dy) as u32;
            if x >= image.width() || y >= image.height() {
                continue;
            }
            _ = image.set_color_at(x, y, color);
        }
    }
}
