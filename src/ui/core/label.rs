use bevy::prelude::*;

#[derive(Component)]
pub struct Label;

pub fn spawn_label<T: Bundle>(commands: &mut Commands, text: &str, components: T) -> Entity {
    commands
        .spawn((
            components,
            Label,
            TextBundle::from_section(
                text,
                TextStyle {
                    font_size: 24.,
                    color: Color::BLACK,
                    ..Default::default()
                },
            ),
        ))
        .id()
}
