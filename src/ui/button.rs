use bevy::{prelude::*, ui::FocusPolicy};

#[derive(Component, Default)]
pub struct MyButton {
    pub enabled: bool,
}

pub fn spawn_button<T: Bundle>(builder: &mut ChildBuilder, title: &str, components: T) {
    builder
        .spawn((
            components,
            MyButton::default(),
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
        .with_children(|builder| {
            builder.spawn((TextBundle::from_section(
                title,
                TextStyle {
                    font_size: 24.,
                    ..Default::default()
                },
            ),));
        });
}
