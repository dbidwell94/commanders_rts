use crate::node_with_padding;
use bevy::prelude::*;
use bevy_ui_dsl::*;

pub struct DropdownMenu {
    pub is_open: bool,
    pub name: String,
    pub items: Vec<String>,
}
