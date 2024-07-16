use bevy::prelude::*;
use commanders_global::game_state::GameState;

#[derive(SubStates, Clone, Eq, PartialEq, Debug, Default, Hash)]
#[source(GameState = GameState::MainMenu)]
pub enum MenuState {
    #[default]
    MainMenu,
    NewGame,
    LoadGame,
    Multiplayer,
    Options,
    Quit,
}
