use crate::{state::MenuState, ButtonAnimation, NextStateValue};
use bevy::prelude::*;
use bevy_ui_dsl::*;
use commanders_global::{components, game_state::GameState};
use ui_lib::padded_button_bundle;

pub fn add_options_systems(app: &mut App) {
    app.add_systems(
        OnEnter(MenuState::Options),
        build_options_menu.run_if(in_state(GameState::MainMenu)),
    )
    .add_systems(
        Update,
        (handle_interactions)
            .run_if(in_state(MenuState::Options))
            .run_if(in_state(GameState::MainMenu)),
    )
    .add_systems(
        OnExit(MenuState::Options),
        destory_options_menu.run_if(in_state(GameState::MainMenu)),
    );
}

components!(BackButton, SettingsUI);

fn build_options_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut back = None;

    let root_entity = root(
        ui_lib::root_full_screen(Some(JustifyContent::Center), Some(AlignItems::Center)),
        &asset_server,
        &mut commands,
        |p| {
            node(ui_lib::node_with_padding(None, None, None, None), p, |p| {
                text_button("Back", padded_button_bundle(), (), p).set(&mut back);
            });
        },
    );

    let back = back.expect("Expected entity to be defined");

    commands.entity(root_entity).insert(SettingsUI);
    commands.entity(back).insert((ButtonAnimation, BackButton));
    commands
        .entity(back)
        .insert(NextStateValue(MenuState::MainMenu));
}

fn handle_interactions(
    mut animation_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ButtonAnimation>),
    >,
    state_query: Query<(&Interaction, &NextStateValue)>,
    mut next_state: ResMut<NextState<MenuState>>,
) {
    // animate
    for (interaction, mut background) in animation_query.iter_mut() {
        match interaction {
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

    // state changes
    for (interaction, state_value) in state_query.iter() {
        match interaction {
            Interaction::Pressed => {
                next_state.set((**state_value).clone());
            }
            _ => {}
        }
    }
}

fn destory_options_menu(mut commands: Commands, query: Query<Entity, With<SettingsUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
