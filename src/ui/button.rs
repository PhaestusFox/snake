use bevy::ecs::system::SystemId;
use bevy::feathers::rounded_corners::RoundedCorners;

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

    match trigger.button {
        PointerButton::Primary if let Some(system) = button_actions.on_primary => {
            commands.run_system(system);
        }
        PointerButton::Middle if let Some(system) = button_actions.on_middle => {
            commands.run_system(system);
        }
        PointerButton::Secondary if let Some(system) = button_actions.on_secondary => {
            commands.run_system(system);
        }
        _ => {}
    }
}

#[derive(Component, Default)]
#[require(Button)]
pub struct ButtonActions {
    on_primary: Option<SystemId>,
    on_middle: Option<SystemId>,
    on_secondary: Option<SystemId>,
}

impl ButtonActions {
    pub fn new() -> Self {
        ButtonActions::default()
    }
    pub fn with_primary(mut self, system: SystemId) -> Self {
        self.on_primary = Some(system);
        self
    }
    pub fn with_middle(mut self, system: SystemId) -> Self {
        self.on_middle = Some(system);
        self
    }
    pub fn with_secondary(mut self, system: SystemId) -> Self {
        self.on_secondary = Some(system);
        self
    }
}

pub fn button<C: bevy::ecs::spawn::SpawnableList<ChildOf> + Send + Sync + 'static, B: Bundle>(
    props: bevy::feathers::controls::ButtonProps,
    overrides: B,
    children: C,
    actions: ButtonActions,
) -> impl Bundle {
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
    (
        Node {
            width: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            padding: UiRect::axes(Val::Px(8.0), Val::Px(0.)),
            flex_grow: 1.0,
            ..Default::default()
        },
        Button,
        props.variant,
        Hovered::default(),
        EntityCursor::System(bevy::window::SystemCursorIcon::Pointer),
        TabIndex(0),
        to_border_radius(props.corners, Val::Vw(1.)),
        ThemeBackgroundColor(tokens::BUTTON_BG),
        ThemeFontColor(tokens::BUTTON_TEXT),
        InheritableFont {
            font: HandleOrPath::Path(fonts::REGULAR.to_owned()),
            font_size: 14.0,
        },
        overrides,
        Children::spawn(children),
        actions,
    )
}

// use bevy::feathers::controls::button;

/// Convert the `RoundedCorners` to a `BorderRadius` for use in a `Node`.
pub fn to_border_radius(corners: RoundedCorners, radius: Val) -> BorderRadius {
    let zero = Val::ZERO;
    match corners {
        RoundedCorners::None => BorderRadius::all(zero),
        RoundedCorners::All => BorderRadius::all(radius),
        RoundedCorners::TopLeft => BorderRadius {
            top_left: radius,
            top_right: zero,
            bottom_right: zero,
            bottom_left: zero,
        },
        RoundedCorners::TopRight => BorderRadius {
            top_left: zero,
            top_right: radius,
            bottom_right: zero,
            bottom_left: zero,
        },
        RoundedCorners::BottomRight => BorderRadius {
            top_left: zero,
            top_right: zero,
            bottom_right: radius,
            bottom_left: zero,
        },
        RoundedCorners::BottomLeft => BorderRadius {
            top_left: zero,
            top_right: zero,
            bottom_right: zero,
            bottom_left: radius,
        },
        RoundedCorners::Top => BorderRadius {
            top_left: radius,
            top_right: radius,
            bottom_right: zero,
            bottom_left: zero,
        },
        RoundedCorners::Right => BorderRadius {
            top_left: zero,
            top_right: radius,
            bottom_right: radius,
            bottom_left: zero,
        },
        RoundedCorners::Bottom => BorderRadius {
            top_left: zero,
            top_right: zero,
            bottom_right: radius,
            bottom_left: radius,
        },
        RoundedCorners::Left => BorderRadius {
            top_left: radius,
            top_right: zero,
            bottom_right: zero,
            bottom_left: radius,
        },
    }
}
