pub use super::button::ButtonActions;
pub use super::slider::SliderActions;
use bevy::{
    ecs::system::SystemId,
    feathers::{FeathersPlugins, dark_theme::create_dark_theme, theme::UiTheme},
    prelude::*,
};

mod main;
mod pause;
mod settings;
mod skins;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<OpenMenu>();
        app.init_resource::<BackMenuAction>();
        app.add_plugins(FeathersPlugins)
            .insert_resource(UiTheme(create_dark_theme()));
        app.init_resource::<MenuTree>()
            .add_systems(OnEnter(OpenMenu::Main), clear_tree);
        app.add_plugins((
            main::MainMenu,
            settings::SettingsMenu,
            pause::PauseMenu,
            skins::SkinsMenu,
        ));
        app.add_systems(Last, update_tree);
    }
}

#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone)]
pub enum OpenMenu {
    #[default]
    Main,
    Settings,
    Pause,
    Skins,
    None,
}

fn default_menu_node() -> Node {
    Node {
        align_items: AlignItems::Stretch,
        flex_direction: FlexDirection::Column,
        margin: UiRect::all(Val::Auto),
        ..Node::DEFAULT
    }
}

#[derive(Resource, Deref, DerefMut, Default)]
struct MenuTree(Vec<OpenMenu>);

fn clear_tree(mut tree: ResMut<MenuTree>) {
    tree.clear();
}

fn update_tree(mut tree: ResMut<MenuTree>, open: Res<State<OpenMenu>>) {
    // if the open menu has not changed, do nothing
    if !open.is_changed() {
        return;
    }
    // get the last open menu in the tree
    // if it is the same as the current open menu, do nothing
    if let Some(last) = tree.last()
        && last == open.get()
    {
        warn!("Menu tree desynced: new menu is the same as previous menu");
        return;
    }

    // push the new open menu to the tree
    tree.push(open.get().clone());
}

fn go_back(mut tree: ResMut<MenuTree>, mut next: ResMut<NextState<OpenMenu>>) {
    // discard the current menu
    tree.pop();
    // get the previous menu
    if let Some(previous) = tree.last() {
        next.set(previous.clone());
    } else {
        next.set(OpenMenu::Main);
    }
}

#[derive(Resource)]
struct BackMenuAction {
    go_back: SystemId,
}
impl FromWorld for BackMenuAction {
    fn from_world(world: &mut World) -> Self {
        Self {
            go_back: world.register_system(go_back),
        }
    }
}
