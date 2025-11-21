use super::*;
use bevy::{
    ecs::system::SystemId,
    feathers::{
        controls::{ButtonProps, ButtonVariant},
        *,
    },
};

pub struct MainMenu;

impl Plugin for MainMenu {
    fn build(&self, app: &mut App) {
        app.init_resource::<MainMenuActions>()
            .add_systems(OnEnter(OpenMenu::Main), spawn_main_menu);
    }
}

#[derive(Resource)]
struct MainMenuActions {
    start_game: SystemId,
    open_settings: SystemId,
    quit_game: SystemId,
}

impl FromWorld for MainMenuActions {
    fn from_world(world: &mut World) -> Self {
        Self {
            start_game: world.register_system(|mut state: ResMut<NextState<OpenMenu>>| {
                state.set(OpenMenu::None);
            }),
            open_settings: world.register_system(|mut state: ResMut<NextState<OpenMenu>>| {
                state.set(OpenMenu::Settings);
            }),
            quit_game: world.register_system(|mut exit: MessageWriter<AppExit>| {
                exit.write(AppExit::Success);
            }),
        }
    }
}

fn spawn_main_menu(mut commands: Commands, actions: Res<MainMenuActions>) {
    debug!("Opening Main Menu");
    // Spawn menu container
    // this will be scoped to the OpenMenu::Main state
    commands
        .spawn((
            Node {
                width: Val::Vw(100. / 3.),
                height: Val::Vh(300. / 5.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                margin: UiRect::all(Val::Auto),
                ..default_menu_node()
            },
            theme::ThemeBackgroundColor(tokens::WINDOW_BG),
            BorderRadius::all(Val::Vw(1.)),
            DespawnOnExit(OpenMenu::Main),
        ))
        // add button to start game
        .with_child(button(
            ButtonProps {
                variant: ButtonVariant::Primary,
                corners: rounded_corners::RoundedCorners::Top,
            },
            (),
            Spawn((Text::from("Start Game"),)),
            ButtonActions::new().with_primary(actions.start_game),
        ))
        // add button to open settings
        .with_child(button(
            ButtonProps {
                variant: ButtonVariant::Normal,
                corners: rounded_corners::RoundedCorners::None,
            },
            (),
            Spawn((Text::from("Settings"),)),
            ButtonActions::new().with_primary(actions.open_settings),
        ))
        // add button to quit game
        .with_child(button(
            ButtonProps {
                variant: ButtonVariant::Normal,
                corners: rounded_corners::RoundedCorners::Bottom,
            },
            (),
            Spawn((Text::from("Quit"),)),
            ButtonActions::new().with_primary(actions.quit_game),
        ));
}
