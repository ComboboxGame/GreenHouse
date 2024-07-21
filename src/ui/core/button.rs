use bevy::{prelude::*, ui::FocusPolicy};

use super::label::spawn_label;

#[derive(Component, Default)]
pub struct Button {
    pub enabled: bool,
}

pub fn spawn_button<T: Bundle>(
    commands: &mut Commands,
    title: &str,
    components: T,
) -> Entity {
    let button = commands
        .spawn((
            components,
            Button::default(),
            Interaction::default(),
            NodeBundle {
                style: Style {
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::WHITE),
                focus_policy: FocusPolicy::Block,
                z_index: ZIndex::Global(25),
                ..Default::default()
            },
            UiImage::default(),
        ))
        .id();

    let label = spawn_label(commands, title, ());

    commands.entity(button).push_children(&[label]);

    button
}
