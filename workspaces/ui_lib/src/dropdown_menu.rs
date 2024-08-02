use bevy::prelude::*;
use bevy_ui_dsl::*;

pub trait DropdownMenuItemExt: ToString + Send + Sync + 'static {}

#[derive(Event, Debug)]
pub struct DropdownMenuEvent<T>(pub T)
where
    T: ToString + Send + Sync;

#[derive(Component)]
pub struct DropdownMenu {
    pub open: bool,
    pub items: Vec<Box<dyn DropdownMenuItemExt>>,
    pub parent: Entity,
}

#[derive(Component)]
pub struct DropdownMenuItem(String);

pub struct DropdownMenuPlugin;

impl Plugin for DropdownMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, open_dropdown_menu);
    }
}

fn open_dropdown_menu(
    mut commands: Commands,
    mut query: Query<(&mut DropdownMenu, &Interaction, Entity)>,
    asset_server: Res<AssetServer>,
) {
    for (mut dropdown_menu, interaction, entity) in query.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                dropdown_menu.open = !dropdown_menu.open;
                spawn_dropdown_menu_children(&mut commands, entity, &asset_server, &dropdown_menu);
            }
            _ => {}
        }
    }
}

fn spawn_dropdown_menu_children(
    commands: &mut Commands,
    entity: Entity,
    asset_server: &Res<AssetServer>,
    dropdown_menu: &DropdownMenu,
) {
    let parent = root(
        |node: &mut NodeBundle| {
            node.style.position_type = PositionType::Absolute;
        },
        &asset_server,
        commands,
        |p| {},
    );
}
