use bevy::{app::{App, Plugin, Startup, Update}, ecs::{component::Component, entity::Entity, system::{Commands, Query, Res}}, state::state::StateTransition, time::Time};

#[derive(Debug)]
pub enum CycleType {
    Solar,
    Water,
    Temperature,
    Wind,
}

#[derive(Debug, Component)]
pub struct Cycle {
    period: f32,
    amplitude: f32,
    value: f32,
    r#type: CycleType,
}

impl Cycle {
    pub fn new(r#type: CycleType) -> Self {
        Self {
            period: 1.,
            amplitude: 1.,
            value: 0.,
            r#type,
        }
    }

    pub fn update(&mut self, time: f32) {
        self.value = self.amplitude * (self.period + time).sin();
    }

    pub fn reset_period(&mut self, period: f32) {
        self.period = period;
    }

    pub fn reset_amplitude(&mut self, amplitude: f32) {
        self.amplitude = amplitude;
    }

    pub fn current_value(&self) -> f32 {
        self.value
    }
}

pub fn spawn_cycles(mut commands: Commands) {
    commands.spawn(Cycle::new(CycleType::Solar));
}

pub fn update_cycles(mut cycles: Query<&mut Cycle>, time: Res<Time>) {
    for mut cycle in cycles.iter_mut() {
        cycle.update(time.elapsed_seconds());
        // println!("Current cycle: {}", cycle.current_value());
    }
}

pub struct CyclePlugin;

impl Plugin for CyclePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_cycles);
        app.add_systems(Update, update_cycles);
    }
}