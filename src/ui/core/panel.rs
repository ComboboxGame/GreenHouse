use bevy::prelude::*;

#[derive(Component)]
struct Panel;

pub fn spawn_panel(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            Panel,
            NodeBundle {
                background_color: BackgroundColor(Color::hsl(32., 32., 32.)),
                ..Default::default()
            },
        ))
        .id()
}
