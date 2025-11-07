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

#[derive(Component)]
pub struct ObjectSize(pub UVec2);
