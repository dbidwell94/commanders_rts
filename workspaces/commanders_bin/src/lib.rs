use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use commanders_global::game_state::GameState;
use commanders_main_menu::MainMenuPlugin;

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MainMenuPlugin)
        .init_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::InitialLoad).continue_to_state(GameState::MainMenu),
        )
        .run();
}
