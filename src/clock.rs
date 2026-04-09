use bevy::prelude::*;
use chrono::Timelike;

#[derive(Message)]
pub struct SecondTick(pub u32);

pub struct ClockSysPlugin;

impl Plugin for ClockSysPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<SecondTick>()
            .add_systems(Update, clock_system);
    }
}

fn clock_system(
    mut last_sec: Local<Option<u32>>, 
    mut message_writer: MessageWriter<SecondTick>,
) {
    let now = chrono::Local::now().second();

    if Some(now) != *last_sec {
        *last_sec = Some(now);
        message_writer.write(SecondTick(now));
        println!("{now}");
    }
}