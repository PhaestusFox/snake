use bevy::{
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    prelude::*,
    window::PrimaryWindow,
};

use crate::{
    GRID_SIZE,
    map::{Map, ObjectSize, mini_map::MiniMapColor},
    snake::{Snake, SnakeSegment, SnakeSize},
};

pub struct CollectablesPlugin;

impl Plugin for CollectablesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CollectableSprites>();
        // Update sprites when collectables are inserted
        app.add_observer(update_collectable_sprites);

        app.add_observer(spawn_food);

        app.add_systems(FixedPostUpdate, get_collisions);
    }
}
// x = 14
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[require(Sprite, Collider)]
#[component(on_add = Self::on_add)]
pub enum Collectable {
    Apple, // 6,17
}

impl Collectable {
    #[inline]
    const fn pos(&self) -> UVec2 {
        match self {
            Collectable::Apple => UVec2::new(6, 17),
        }
    }

    const fn index(&self) -> usize {
        match self {
            Collectable::Apple => 6 + 17 * 14,
        }
    }

    fn on_add(mut world: DeferredWorld, ctx: HookContext) {
        // todo set color on minimap based of Self
    }
}

#[derive(Component, Default)]
#[require(ObjectSize)]
pub struct Collider;

fn get_collisions(
    snakes: Query<(Entity, &SnakeSize, &Children), With<Snake>>,
    collectables: Query<(Entity, &Transform, &ObjectSize), (With<Collectable>, With<Collider>)>,
    segments: Query<&GlobalTransform, Without<Snake>>,
    mut commands: Commands,
) {
    for (entity, size, body) in snakes.iter() {
        let size = **size;
        let Some(&head) = body.first() else {
            warn!("Snake has no segments");
            continue;
        };
        let Ok(snake) = segments.get(head) else {
            warn!("Failed to get head segment transform");
            continue;
        };
        for (c_entity, c_pos, collider) in &collectables {
            if check_aabb_collision(
                snake.translation(),
                Vec2::splat(size / 2.),
                c_pos.translation,
                collider.size() / 2.,
            ) {
                commands.entity(c_entity).despawn();
                commands.trigger(SpawnFood);
                commands.entity(entity).with_child(SnakeSegment);
            }
        }
    }
}

fn check_aabb_collision(pos_a: Vec3, half_size_a: Vec2, pos_b: Vec3, half_size_b: Vec2) -> bool {
    let delta_x = (pos_a.x - pos_b.x).abs();
    let delta_y = (pos_a.y - pos_b.y).abs();

    delta_x < (half_size_a.x + half_size_b.x) && delta_y < (half_size_a.y + half_size_b.y)
}

#[derive(Resource)]
struct CollectableSprites {
    root: Handle<Image>,
    layout: Handle<TextureAtlasLayout>,
}

impl FromWorld for CollectableSprites {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        let layout = asset_server.add(TextureAtlasLayout::from_grid(
            UVec2::new(32, 32),
            14,
            34,
            None,
            None,
        ));
        CollectableSprites {
            root: asset_server.load("MegaPixelArt32x32pxIcons_SpriteSheet.png"),
            layout,
        }
    }
}

#[derive(Event)]
pub struct SpawnFood;

fn spawn_food(_: On<SpawnFood>, mut commands: Commands, map: Res<Map>) {
    let rx = map.size().x / 2;
    let ry = map.size().y / 2;
    let x = rand::random_range(-rx..rx);
    let y = rand::random_range(-ry..ry);
    let size = ObjectSize::new(UVec2::splat(2));
    let pos = IVec2::new(x, y).as_vec2() * GRID_SIZE + size.offset();
    commands.spawn((
        Collectable::Apple,
        Transform::from_translation(pos.extend(0.0)),
        ObjectSize::new(UVec2::splat(2)),
        MiniMapColor(Color::linear_rgb(0.1, 0.8, 0.1)),
    ));
}

// this could probably be done better with an on add hook since why would collectable change?
fn update_collectable_sprites(
    added: On<Insert, Collectable>,
    mut collectables: Query<(&mut Sprite, &Collectable)>,
    sprites: Res<CollectableSprites>,
) {
    let Ok((mut sprite, collectable)) = collectables.get_mut(added.entity) else {
        warn!("Failed to get collectable sprite for newly added collectable");
        return;
    };
    sprite.texture_atlas = Some(TextureAtlas {
        index: collectable.index(),
        layout: sprites.layout.clone(),
    });
    sprite.image = sprites.root.clone();
}
