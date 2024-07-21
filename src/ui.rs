use bevy::prelude::*;
use cycle::{spawn_cycle_menu, update_cycle_controls};

mod core;
mod cycle;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_game_ui);

        app.add_systems(Update, update_cycle_controls);
    }
}

#[derive(Component)]
struct MainUiNode;

fn spawn_game_ui(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), IsDefaultUiCamera));

    let main_ui_node = spawn_main_ui_node(&mut commands);
    let panel = spawn_cycle_menu(&mut commands);
    commands.entity(main_ui_node).push_children(&[panel]);
}

fn spawn_main_ui_node(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    ..Default::default()
                },
                ..Default::default()
            },
            MainUiNode {},
            Interaction::None,
        ))
        .id()
}
