use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use commanders_global::{game_state::GameState, ExitEvent};
use commanders_main_menu::MainMenuPlugin;

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MainMenuPlugin)
        .observe(observe_exit_event)
        .init_state::<GameState>()
        .enable_state_scoped_entities::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::InitialLoad).continue_to_state(GameState::MainMenu),
        )
        .run();
}

fn observe_exit_event(trigger: Trigger<ExitEvent>, mut exit_event_writer: ResMut<Events<AppExit>>) {
    let exit_reason = trigger
        .event()
        .0
        .clone()
        .unwrap_or("No exit reason given".into());
    info!("Exit event triggered: {}", exit_reason);

    exit_event_writer.send(AppExit::Success);
}
