use crate::snake::{SnakeId, SnakeSkins};

use super::*;
use bevy::{
    ecs::system::SystemId,
    feathers::{
        controls::{ButtonProps, ButtonVariant, button},
        *,
    },
    ui::InteractionDisabled,
};
use strum::{EnumCount, IntoEnumIterator};

pub struct SkinsMenu;

impl Plugin for SkinsMenu {
    fn build(&self, app: &mut App) {
        app.init_resource::<MenuActions>()
            .add_systems(OnEnter(OpenMenu::Skins), spawn_menu);
    }
}

#[derive(Resource)]
struct MenuActions {
    change_skin: SystemId<In<Entity>>,
    back: SystemId,
}

impl FromWorld for MenuActions {
    fn from_world(world: &mut World) -> Self {
        Self {
            change_skin: world.register_system(
                |snake_id: In<Entity>,
                 mut skins: ResMut<SnakeSkins>,
                 thing_with_skins: Query<&SnakeId, With<Button>>| {
                    let Ok(skin) = thing_with_skins.get(snake_id.0) else {
                        warn!(
                            "Tried to change skin with invalid button entity: {:?}",
                            snake_id.0
                        );
                        return;
                    };
                    debug!("Changing skin to {:?}", snake_id.0);
                    skins.set_active_skin(*skin);
                },
            ),
            back: world.resource::<BackMenuAction>().go_back,
        }
    }
}

fn spawn_menu(mut commands: Commands, actions: Res<MenuActions>, skins: Res<SnakeSkins>) {
    debug!("Opening Skins Menu");
    let r = (SnakeId::COUNT as f32 + 1.).sqrt().ceil();
    // Spawn menu container
    // this will be scoped to the OpenMenu::Skins state
    commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_content: AlignContent::Stretch,
                flex_wrap: FlexWrap::Wrap,
                flex_direction: FlexDirection::Row,
                margin: UiRect::all(Val::Auto),
                display: Display::Grid,
                grid_auto_flow: GridAutoFlow::Column,
                grid_auto_rows: vec![GridTrack::percent(100. / r)],
                grid_auto_columns: vec![GridTrack::percent(100. / r)],
                grid_template_columns: vec![GridTrack::percent(100. / r); r as usize],
                grid_template_rows: vec![GridTrack::percent(100. / r); r as usize],
                ..default_menu_node()
            },
            theme::ThemeBackgroundColor(tokens::WINDOW_BG),
            BorderRadius::all(Val::Vw(1.)),
            DespawnOnExit(OpenMenu::Skins),
        ))
        .with_children(|p| {
            for skin in SnakeId::iter() {
                let mut s = p.spawn((button(
                    ButtonProps {
                        variant: if skin == skins.active_skin() {
                            ButtonVariant::Primary
                        } else {
                            ButtonVariant::Normal
                        },
                        corners: rounded_corners::RoundedCorners::All,
                    },
                    (ButtonActions::new().with_primary(actions.change_skin), skin),
                    Spawn(Text::from(format!("{:?}", skin))),
                ),));
                s.insert(Node {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    padding: UiRect::axes(Val::Px(8.0), Val::Px(0.)),
                    flex_grow: 1.0,
                    ..Default::default()
                });
                if !skins.unlocked(skin) {
                    s.insert(InteractionDisabled);
                }
            }
            // add button to go back
            p.spawn(button(
                ButtonProps {
                    variant: ButtonVariant::Primary,
                    corners: rounded_corners::RoundedCorners::All,
                },
                ButtonActions::new().with_primary(actions.back),
                Spawn(Text::from("Back")),
            ))
            .insert(Node {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::axes(Val::Px(8.0), Val::Px(0.)),
                flex_grow: 1.0,
                grid_row: GridPlacement::end(-1),
                grid_column: GridPlacement::end(-1),
                ..Default::default()
            });
        });
}
