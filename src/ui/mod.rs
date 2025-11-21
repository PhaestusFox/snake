mod menus;
use bevy::prelude::*;

pub use menus::MenuPlugin;
pub use menus::OpenMenu;

mod button;

use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MenuPlugin);
        app.add_plugins(button::ButtonPlugin);
    }
}
