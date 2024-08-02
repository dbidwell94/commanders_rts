mod dropdown_menu;
use bevy::prelude::*;

/// Display: Flex centered on screen
pub fn root_full_screen(
    justify: Option<JustifyContent>,
    align: Option<AlignItems>,
) -> impl Fn(&mut NodeBundle) {
    move |node: &mut NodeBundle| {
        node.style = Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: justify.unwrap_or(JustifyContent::Center),
            align_items: align.unwrap_or(AlignItems::Center),
            ..default()
        };
    }
}

pub fn node_with_padding(
    padding: Option<UiRect>,
    direction: Option<FlexDirection>,
    justify: Option<JustifyContent>,
    align: Option<AlignItems>,
) -> impl Fn(&mut NodeBundle) {
    move |node: &mut NodeBundle| {
        node.style = Style {
            display: Display::Flex,
            flex_direction: direction.unwrap_or(FlexDirection::Column),
            justify_content: justify.unwrap_or(JustifyContent::Center),
            align_items: align.unwrap_or(AlignItems::Center),
            padding: padding.unwrap_or(UiRect {
                bottom: Val::Px(5.),
                top: Val::Px(5.),
                left: Val::Px(5.),
                right: Val::Px(5.),
            }),
            // todo: remove this. This is a debug placeholder to view the layout
            border: UiRect {
                bottom: Val::Px(1.),
                top: Val::Px(1.),
                left: Val::Px(1.),
                right: Val::Px(1.),
            },
            ..default()
        };

        // todo: remove this. This is a debug placeholder to view the layout
        node.border_color = BorderColor(Color::WHITE);
    }
}

pub fn padded_button_bundle() -> impl Fn(&AssetServer, &mut ButtonBundle) {
    move |_: &AssetServer, button: &mut ButtonBundle| {
        button.style = Style {
            padding: UiRect {
                bottom: Val::Px(5.),
                top: Val::Px(5.),
                left: Val::Px(5.),
                right: Val::Px(5.),
            },
            ..default()
        };
    }
}
