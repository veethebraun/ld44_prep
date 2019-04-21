
use amethyst::ecs::{Read, System };
use amethyst::input::InputHandler;
use crate::audio::*;

pub struct SoundFxSystem;
impl<'s> System<'s> for SoundFxSystem {
    type SystemData = (
        Read<'s, InputHandler<String, String>>,
        AudioSystemData<'s>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (input, audio_system_data) = data;

        if let Some(act) = input.action_is_down("hi") {
            if act {
                println!("Playing: {}", act);
                play_bounce(audio_system_data);
            }
        }
    }

}
