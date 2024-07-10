use bevy::prelude::*;

#[derive(States, Clone, Eq, PartialEq, Debug, Default, Hash)]
pub enum MenuState {
    #[default]
    None,
    MainMenu,
    NewGame,
    LoadGame,
    Multiplayer,
    Options,
    Quit,
}
