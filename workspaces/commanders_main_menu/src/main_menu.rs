use crate::{state::MenuState, ButtonAnimation, NextStateValue};
use bevy::prelude::*;
use bevy_ui_dsl::*;
use commanders_global::{components, game_state::GameState};
use ui_lib::{node_with_padding, padded_button_bundle, root_full_screen};

pub fn add_main_menu_systems(app: &mut App) {
    app.add_plugins(bevy::dev_tools::ui_debug_overlay::DebugUiPlugin)
        .add_systems(
            OnEnter(MenuState::MainMenu),
            build_main_menu.run_if(in_state(GameState::MainMenu)),
        )
        .add_systems(
            Update,
            (handle_button_interaction)
                .run_if(in_state(GameState::MainMenu))
                .run_if(in_state(MenuState::MainMenu)),
        )
        .add_systems(
            OnExit(MenuState::MainMenu),
            destroy_main_menu.run_if(in_state(GameState::MainMenu)),
        );
}

components!(
    NewGameButton,
    LoadGameButton,
    MultiplayerButton,
    SettingsButton,
    ExitButton,
    MainMenuUI
);

fn build_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    camera_query: Query<Entity, With<Camera>>,
) {
    // create camera if it does not exist
    if camera_query.iter().count() == 0 {
        commands.spawn(Camera3dBundle::default());
    }

    let mut new_game = None;
    let mut load_game = None;
    let mut multiplayer = None;
    let mut settings = None;
    let mut exit = None;

    let root_entity = root(
        root_full_screen(Some(JustifyContent::Center), Some(AlignItems::Center)),
        &asset_server,
        &mut commands,
        |p| {
            node(node_with_padding(None, None, None, None), p, |p| {
                text_button("New Game", padded_button_bundle(), (), p).set(&mut new_game);
                text_button("Load Game", padded_button_bundle(), (), p).set(&mut load_game);
                text_button("Multiplayer", padded_button_bundle(), (), p).set(&mut multiplayer);
                text_button("Settings", padded_button_bundle(), (), p).set(&mut settings);
                text_button("Exit", padded_button_bundle(), (), p).set(&mut exit);
            });
        },
    );

    let (new_game, load_game, multiplayer, settings, exit) = (
        new_game.expect("Expected entity to be defined"),
        load_game.expect("Expected entity to be defined"),
        multiplayer.expect("Expected entity to be defined"),
        settings.expect("Expected entity to be defined"),
        exit.expect("Expected entity to be defined"),
    );

    commands.entity(new_game).insert((
        NewGameButton,
        ButtonAnimation,
        NextStateValue(MenuState::NewGame),
    ));
    commands.entity(load_game).insert((
        LoadGameButton,
        ButtonAnimation,
        NextStateValue(MenuState::LoadGame),
    ));
    commands.entity(multiplayer).insert((
        MultiplayerButton,
        ButtonAnimation,
        NextStateValue(MenuState::Multiplayer),
    ));
    commands.entity(settings).insert((
        SettingsButton,
        ButtonAnimation,
        NextStateValue(MenuState::Options),
    ));

    commands.entity(exit).insert((ExitButton, ButtonAnimation));

    commands
        .entity(root_entity)
        .insert((MainMenuUI, StateScoped(MenuState::MainMenu)));
}

fn handle_button_interaction(
    mut animation_query: Query<
        (&mut BackgroundColor, &Interaction),
        (Changed<Interaction>, With<ButtonAnimation>),
    >,
    next_state_query: Query<(&NextStateValue, &Interaction)>,
    mut next_state: ResMut<NextState<MenuState>>,
) {
    // handle animation
    for (mut background, interaction) in animation_query.iter_mut() {
        match *interaction {
            Interaction::Hovered => {
                background.0 = Color::srgb(0.5, 0.5, 0.5);
            }
            Interaction::Pressed => {
                background.0 = Color::srgb(0.3, 0.3, 0.3);
            }
            Interaction::None => {
                background.0 = Color::NONE;
            }
        }
    }

    // handle next state
    for (next_state_value, interaction) in next_state_query.iter() {
        if *interaction == Interaction::Pressed {
            next_state.set((**next_state_value).clone());
        }
    }
}

fn destroy_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenuUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
