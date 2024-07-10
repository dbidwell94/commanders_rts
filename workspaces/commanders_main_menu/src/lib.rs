mod main_menu;
mod options;
mod state;

use bevy::prelude::*;
use commanders_global::game_state::GameState;
use state::MenuState;

pub struct MainMenuPlugin;

#[derive(Component)]
pub struct ButtonAnimation;

#[derive(Component, Deref)]
pub struct NextStateValue(MenuState);

#[derive(Component)]
struct UiCamera;

impl Plugin for MainMenuPlugin {
    fn build(&self, mut app: &mut App) {
        // start -- Not in menu state
        app.init_state::<MenuState>()
            .add_systems(OnEnter(GameState::MainMenu), init_menu_states);

        // 1 -- main menu
        main_menu::add_main_menu_systems(&mut app);

        // 2 -- options menu
        options::add_options_systems(&mut app);

        // end -- Not in menu state
        app.add_systems(OnExit(GameState::MainMenu), reset_menu_states);
    }
}

fn init_menu_states(
    mut next_state: ResMut<NextState<MenuState>>,
    mut commands: Commands,
    camera_query: Query<Entity, With<UiCamera>>,
) {
    next_state.set(MenuState::MainMenu);

    if camera_query.iter().next().is_some() {
        return;
    }

    // build a 3D camera so the UI can be rendered.
    // todo: add 3d gameplay in background of main menu
    commands.spawn((Camera3dBundle::default(), UiCamera));
}

fn reset_menu_states(
    mut next_state: ResMut<NextState<MenuState>>,
    mut commands: Commands,
    camera_query: Query<Entity, With<UiCamera>>,
) {
    next_state.set(MenuState::None);

    // Remove the temporary 3D ui camera in favor of the main camera (in other workspace)
    if let Some(camera) = camera_query.iter().next() {
        commands.entity(camera).despawn_recursive();
    }
}
