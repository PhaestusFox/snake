mod menus;
use bevy::feathers::rounded_corners::RoundedCorners;
use bevy::prelude::*;

pub use menus::MenuPlugin;
pub use menus::OpenMenu;

mod button;
mod slider;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MenuPlugin);
        app.add_plugins(button::ButtonPlugin);
        app.add_plugins(slider::SliderPlugin);
    }
}

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
