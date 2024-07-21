use bevy::{ecs::entity, prelude::*};

use crate::{
    game::cycle::{Cycle, CycleType},
    ui::core::spawn_label,
};

#[derive(Component)]
pub struct Control;

#[derive(Component)]
pub struct CurrentValue {
    r#type: CycleType,
}

pub fn spawn_cycle_control(commands: &mut Commands, cycle_type: CycleType) -> Entity {
    let base = commands
        .spawn((
            Control,
            NodeBundle {
                background_color: BackgroundColor(Color::linear_rgb(145., 221., 64.)),
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Px(100.),
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .id();

    let label = spawn_label(commands, &format!("{} cycle", cycle_type.to_string()), ());
    let curr_value = spawn_current_value(commands, cycle_type);
    
    commands
        .entity(base)
        .push_children(&[label, curr_value])
        .id()
}

pub fn spawn_current_value(commands: &mut Commands, cycle_type: CycleType) -> Entity {
    let base = commands
        .spawn((NodeBundle {
            ..Default::default()
        },))
        .id();

    let label = spawn_label(commands, "Current value:", ());
    let current_value = spawn_label(commands, "", CurrentValue {r#type: cycle_type});

    commands
        .entity(base)
        .push_children(&[label, current_value])
        .id()
}

pub fn update_cycle_controls(
    cycles: Query<&Cycle>,
    mut controls: Query<(&CurrentValue, &mut Text)>,
) {
    for cycle in cycles.iter() {
        for (control, mut text) in controls.iter_mut() {
            if cycle.cycle_type() == control.r#type {
                text.sections[0].value = format!("{:.1}", cycle.current_value());
            }
        }
    }
}
