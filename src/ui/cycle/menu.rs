use bevy::prelude::*;

use crate::game::cycle::CycleType;

use super::control::spawn_cycle_control;

#[derive(Component)]
pub struct CycleMenu;

pub fn spawn_cycle_menu(commands: &mut Commands) -> Entity {
    let menu = commands
        .spawn((
            CycleMenu,
            NodeBundle {
                style: Style {
                    width: Val::Percent(30.),
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .id();
    let cycle = spawn_cycle_control(commands, CycleType::Solar);
    commands.entity(menu).push_children(&[cycle]).id()
}
