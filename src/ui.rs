use bevy::prelude::*;

use self::button::spawn_button;

mod button;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Last, spawn_game_ui);
    }
}

#[derive(Component)]
struct MainUiNode;

fn spawn_game_ui(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), IsDefaultUiCamera));
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
        .with_children(|builder| {
            spawn_button(builder, "Button1", ());
        });
}
