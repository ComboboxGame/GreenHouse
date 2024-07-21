use bevy::{app::{App, Plugin}, time::Time};

use self::cycle::CyclePlugin;

pub mod cycle;

pub struct Game;

impl Plugin for Game {
    fn build(&self, app: &mut App) {
        app.init_resource::<Time>();
        app.add_plugins(CyclePlugin{});
    }
}
