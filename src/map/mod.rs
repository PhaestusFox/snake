use bevy::ecs::{lifecycle::HookContext, world::DeferredWorld};

use super::*;

pub mod mini_map;

#[derive(Resource)]
pub struct Map {
    size: UVec2,
}

impl Map {
    pub fn size(&self) -> IVec2 {
        self.size.as_ivec2()
    }
}

impl FromWorld for Map {
    fn from_world(_world: &mut World) -> Self {
        Map {
            size: UVec2::new(100, 100),
        }
    }
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Map>();
        app.add_plugins(mini_map::MiniMapPlugin);
    }
}

#[derive(Component, Clone, Copy)]
#[component(on_add = Self::on_add)]
pub struct ObjectSize(pub UVec2);

impl Default for ObjectSize {
    fn default() -> Self {
        ObjectSize(UVec2::splat(1))
    }
}

impl ObjectSize {
    pub fn size(&self) -> Vec2 {
        self.0.as_vec2() * crate::WORLD_GRID_SIZE
    }

    fn on_add(mut world: DeferredWorld, ctx: HookContext) {
        let size = *world
            .get::<Self>(ctx.entity)
            .expect("Collectable requires Collider");
        if let Some(mut sprite) = world.get_mut::<Sprite>(ctx.entity) {
            sprite.custom_size = Some(size.size());
        }
    }

    pub fn new(size: UVec2) -> Self {
        ObjectSize(size)
    }
}
