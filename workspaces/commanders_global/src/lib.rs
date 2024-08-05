use bevy::prelude::*;

pub mod game_state;

#[macro_export]
macro_rules! component_markers {
    ($($name:ident),*) => {
        $(
            #[derive(Component)]
            struct $name;
        )*
    }
}

#[derive(Event, Debug)]
pub struct ExitEvent(pub Option<String>);
