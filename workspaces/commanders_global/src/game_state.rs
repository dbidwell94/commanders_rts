use bevy::prelude::*;

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    InitialLoad,
    MainMenu,
    SinglePlayer,
    MultiPlayer,
}
