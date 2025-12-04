use std::time::Duration;

use super::*;
use bevy::{
    ecs::system::SystemId,
    feathers::{
        controls::{ButtonProps, ButtonVariant, SliderProps, button, slider},
        theme::ThemeBackgroundColor,
        *,
    },
    ui_widgets::{SliderPrecision, SliderStep, observe},
};

pub struct SettingsMenu;

impl Plugin for SettingsMenu {
    fn build(&self, app: &mut App) {
        app.init_resource::<MenuActions>()
            .add_systems(OnEnter(OpenMenu::Settings), spawn_menu);
    }
}

#[derive(Resource)]
struct MenuActions {
    back: SystemId,
    change_tick_speed: SystemId<In<f32>>,
    open_skins: SystemId,
}

impl FromWorld for MenuActions {
    fn from_world(world: &mut World) -> Self {
        let back = world.resource::<BackMenuAction>().go_back;
        Self {
            back,
            change_tick_speed: world.register_system(
                |new_speed: In<f32>, mut time: ResMut<Time<Fixed>>| {
                    debug!("Changing tick speed to {}", new_speed.0);
                    time.set_timestep(Duration::from_secs_f32(new_speed.0));
                },
            ),
            open_skins: world.register_system(|mut state: ResMut<NextState<OpenMenu>>| {
                state.set(OpenMenu::Skins);
            }),
        }
    }
}

fn spawn_menu(mut commands: Commands, actions: Res<MenuActions>) {
    debug!("Opening Settings Menu");
    // Spawn menu container
    // this will be scoped to the OpenMenu::Settings state
    commands.spawn((
        Node {
            width: Val::Vw(100. / 3.),
            height: Val::Vh(300. / 5.),
            justify_content: JustifyContent::Center,
            align_content: AlignContent::Stretch,
            flex_wrap: FlexWrap::Wrap,
            ..default_menu_node()
        },
        theme::ThemeBackgroundColor(tokens::WINDOW_BG),
        BorderRadius::all(Val::Vw(1.)),
        DespawnOnExit(OpenMenu::Settings),
        children![
            // add slider to change tick speed
            (
                Node {
                    padding: UiRect::axes(Val::Px(8.), Val::Px(4.)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    align_content: AlignContent::Stretch,
                    min_height: Val::Percent(10.),
                    ..Default::default()
                },
                BorderRadius::all(Val::Vw(0.5)),
                ThemeBackgroundColor(tokens::SLIDER_BAR_DISABLED),
                children![
                    (
                        Node {
                            ..Default::default()
                        },
                        Text::from("Tick Speed:"),
                    ),
                    (
                        slider(
                            SliderProps {
                                value: 0.5,
                                max: 1.,
                                min: 0.1,
                            },
                            SliderActions::new().with_on_change(actions.change_tick_speed),
                        ),
                        observe(bevy::ui_widgets::slider_self_update),
                        SliderPrecision(1),
                        SliderStep(0.1),
                    )
                ],
            ),
            (button(
                ButtonProps {
                    variant: ButtonVariant::Normal,
                    corners: rounded_corners::RoundedCorners::All,
                },
                ButtonActions::new().with_primary(actions.open_skins),
                Spawn(Text::from("Skins"))
            )),
            // add button to go back to main menu
            (button(
                ButtonProps {
                    variant: ButtonVariant::Normal,
                    corners: rounded_corners::RoundedCorners::All,
                },
                ButtonActions::new().with_primary(actions.back),
                Spawn(Text::from("Back")),
            )),
        ],
    ));
}
