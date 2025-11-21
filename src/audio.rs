use super::*;

use bevy::{audio::SpatialScale, prelude::*};
use rand::seq::IndexedRandom;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(hiss_on_click).init_resource::<HissSound>();
    }
}

#[derive(Resource, Deref)]
struct HissSound(Vec<Handle<AudioSource>>);

impl FromWorld for HissSound {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        HissSound(vec![
            asset_server.load("sounds/hiss0.wav"),
            asset_server.load("sounds/hiss1.wav"),
            asset_server.load("sounds/hiss2.wav"),
            asset_server.load("sounds/hiss3.wav"),
            asset_server.load("sounds/hiss4.wav"),
            asset_server.load("sounds/hiss5.wav"),
            asset_server.load("sounds/hiss6.wav"),
        ])
    }
}

fn hiss_on_click(click: On<Pointer<Click>>, mut commands: Commands, hiss_sound: Res<HissSound>) {
    if click.button == PointerButton::Primary {
        commands.spawn((
            AudioPlayer::new(hiss_sound.0.choose(&mut rand::rng()).cloned().unwrap()),
            PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Despawn,
                volume: bevy::audio::Volume::Linear(0.5),
                spatial: true,
                speed: rand::random_range(1.2..2.),
                spatial_scale: Some(SpatialScale::new_2d(2.0)),
                ..Default::default()
            },
            Transform::from_translation(click.hit.position.unwrap_or(Vec3::ZERO)),
        ));
    }
}
