mod main_menu;
mod options;
mod state;

use bevy::prelude::*;
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
        app.add_sub_state::<MenuState>()
            .enable_state_scoped_entities::<MenuState>();

        // 1 -- main menu
        main_menu::add_main_menu_systems(&mut app);

        // 2 -- options menu
        options::add_options_systems(&mut app);

        // end -- Not in menu state
    }
}
