pub use super::button::{ButtonActions, button};
use bevy::{
    feathers::{FeathersPlugins, dark_theme::create_dark_theme, theme::UiTheme},
    prelude::*,
};

mod main;
mod pause;
mod settings;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<OpenMenu>();
        app.add_plugins(FeathersPlugins)
            .insert_resource(UiTheme(create_dark_theme()));
        app.add_plugins(main::MainMenu);
    }
}

#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone)]
pub enum OpenMenu {
    #[default]
    Main,
    Settings,
    Pause,
    None,
}

fn default_menu_node() -> Node {
    Node {
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        flex_direction: FlexDirection::Column,
        margin: UiRect::all(Val::Auto),
        ..Node::DEFAULT
    }
}
