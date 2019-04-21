#![allow(non_snake_case)]

extern crate amethyst;

#[macro_use]
extern crate specs_derive;

use amethyst::core::transform::TransformBundle;
use amethyst::prelude::*;
use amethyst::renderer::{DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage};
use amethyst::ui::UiBundle;
use amethyst::audio::AudioBundle;

mod pong;
use crate::pong::Pong;

mod pausable_game_data;
mod systems;
use pausable_game_data::PausableGameDataBuilder;
mod pause_screen;
mod audio;
use audio::Music;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    use amethyst::utils::application_root_dir;
    let path = format!("{}/resources/display_config.ron", application_root_dir());

    let config = DisplayConfig::load(&path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawFlat2D::new())
            .with_pass(amethyst::ui::DrawUi::new()),
    );

    use amethyst::input::InputBundle;

    let binding_path = format!("{}/resources/bindings_config.ron", application_root_dir());

    let input_bundle =
        InputBundle::<String, String>::new().with_bindings_from_file(binding_path)?;

    let game_data = PausableGameDataBuilder::default()
        .with_base_bundle(RenderBundle::new(pipe, Some(config)).with_sprite_sheet_processor())?
        .with_base_bundle(TransformBundle::new())?
        .with_base_bundle(AudioBundle::new(|music: &mut Music| music.music.next()))? //|music: &mut Music| music.music.next()))?
        .with_running_bundle(input_bundle)?
        .with_base_bundle(UiBundle::<String, String>::new())?
        .with_running(systems::PaddleSystem, "paddle_system", &["input_system"])
        .with_running(systems::MoveBallSystem, "move_ball", &[])
        .with_running(systems::SoundFxSystem, "sound_fx", &["input_system"]);

    let mut game = Application::new("./", Pong, game_data)?;

    game.run();

    Ok(())
}
