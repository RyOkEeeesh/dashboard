use bevy::prelude::*;
use chrono::Timelike;

#[derive(EntityEvent)]
pub struct SecondTick(pub u32);

pub struct ClockSysPlugin;

impl Plugin for ClockSysPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, clock_system);
    }
}

fn clock_system(
    mut last_sec: Local<Option<u32>>, 
    mut commands: Commands,
) {
    let now = chrono::Local::now().second();

    if Some(now) != *last_sec {
        *last_sec = Some(now);
        commands.trigger(SecondTick(now));
    }
}