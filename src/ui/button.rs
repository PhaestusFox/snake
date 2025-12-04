use bevy::ecs::system::SystemId;
use bevy::feathers::rounded_corners::RoundedCorners;

use bevy::{
    feathers::{
        constants::fonts,
        cursor::EntityCursor,
        font_styles::InheritableFont,
        handle_or_path::HandleOrPath,
        theme::{ThemeBackgroundColor, ThemeFontColor},
        tokens,
    },
    input_focus::tab_navigation::TabIndex,
    picking::hover::Hovered,
    ui::{AlignItems, JustifyContent, Node, UiRect},
};

use super::*;

pub struct ButtonPlugin;

impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(fire_button_click);
    }
}

fn fire_button_click(
    trigger: On<Pointer<Click>>,
    actions: Query<&ButtonActions>,
    mut commands: Commands,
) {
    let Ok(button_actions) = actions.get(trigger.entity) else {
        return;
    };

    let action = match trigger.button {
        PointerButton::Primary => &button_actions.on_primary,
        PointerButton::Middle => &button_actions.on_middle,
        PointerButton::Secondary => &button_actions.on_secondary,
    };

    match action {
        ButtonAction::None => {}
        ButtonAction::Global(system_id) => {
            commands.run_system(*system_id);
        }
        ButtonAction::Targeted(system_id) => {
            commands.run_system_with(*system_id, trigger.entity);
        }
    }
}

#[derive(Component, Default)]
#[require(Button)]
pub struct ButtonActions {
    on_primary: ButtonAction,
    on_middle: ButtonAction,
    on_secondary: ButtonAction,
}

impl ButtonActions {
    pub fn new() -> Self {
        ButtonActions::default()
    }
    pub fn with_primary(mut self, action: impl Into<ButtonAction>) -> Self {
        self.on_primary = action.into();
        self
    }
    pub fn with_middle(mut self, action: impl Into<ButtonAction>) -> Self {
        self.on_middle = action.into();
        self
    }
    pub fn with_secondary(mut self, action: impl Into<ButtonAction>) -> Self {
        self.on_secondary = action.into();
        self
    }
}

#[derive(Default)]
pub enum ButtonAction {
    #[default]
    None,
    Global(SystemId),
    Targeted(SystemId<In<Entity>>),
}

impl From<SystemId> for ButtonAction {
    fn from(system_id: SystemId) -> Self {
        ButtonAction::Global(system_id)
    }
}

impl From<SystemId<In<Entity>>> for ButtonAction {
    fn from(system_id: SystemId<In<Entity>>) -> Self {
        ButtonAction::Targeted(system_id)
    }
}
