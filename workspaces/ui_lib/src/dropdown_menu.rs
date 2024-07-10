use bevy::prelude::*;
use bevy_ui_dsl::*;

use crate::node_with_padding;

pub struct DropdownMenuPlugin;

#[derive(Component)]
pub struct DropdownMenu<T>(pub T)
where
    T: ToString + Default + Clone;

#[derive(Component)]
struct MenuItem<T>(pub T)
where
    T: ToString + Default + Clone;

#[derive(Component)]
struct MenuOpened(bool);

impl Plugin for DropdownMenuPlugin {
    fn build(&self, app: &mut App) {}
}

pub fn build_dropdown_menu<T: ToString + Default + Clone>(
    mut commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    items: Vec<T>,
) {
    let mut menu_items: Vec<(Option<Entity>, MenuItem<T>)> = items
        .iter()
        .map(|item| (None, MenuItem(item.clone())))
        .collect();

    let root_node = root((), &asset_server, &mut commands, |p| {
        node(
            node_with_padding(
                None,
                Some(FlexDirection::Column),
                Some(JustifyContent::Center),
                Some(AlignItems::Center),
            ),
            p,
            |p| {
                for (index, item) in items.iter().enumerate() {
                    let (mut menu_item, _) = menu_items.get_mut(index).unwrap();
                    text_button(item.to_string(), (), (), p).set(&mut menu_item);
                }
            },
        );
    });
}
