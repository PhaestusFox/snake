use super::*;

use bevy::{
    ecs::system::SystemId,
    feathers::{
        constants::fonts,
        controls::SliderProps,
        cursor::EntityCursor,
        font_styles::InheritableFont,
        handle_or_path::HandleOrPath,
        theme::{ThemeBackgroundColor, ThemeFontColor},
        tokens,
    },
    input_focus::tab_navigation::TabIndex,
    picking::hover::Hovered,
    ui::{AlignItems, JustifyContent, Node, UiRect},
    ui_widgets::{
        SetSliderValue, Slider, SliderRange, SliderValue, SliderValueChange, TrackClick,
        ValueChange,
    },
};

pub struct SliderPlugin;

impl Plugin for SliderPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(fire_slider_changed);
    }
}

#[derive(Component)]
pub struct SliderActions {
    on_change: Option<SystemId<In<f32>>>,
}

impl SliderActions {
    pub fn new() -> Self {
        SliderActions { on_change: None }
    }
    pub fn with_on_change(mut self, system: SystemId<In<f32>>) -> Self {
        self.on_change = Some(system);
        self
    }
}

fn fire_slider_changed(
    events: On<ValueChange<f32>>,
    slider: Query<&SliderActions>,
    mut commands: Commands,
) {
    let Ok(action) = slider.get(events.source) else {
        return;
    };
    if let Some(system) = action.on_change {
        commands.run_system_with(system, events.value);
    }
}
