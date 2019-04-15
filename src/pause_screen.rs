use amethyst::input::{is_close_requested, is_key_down};
use amethyst::prelude::*;

use crate::pausable_game_data::PausableGameData;
use amethyst::assets::Loader;
use amethyst::ecs::prelude::{Component, Join, NullStorage};
use amethyst::renderer::VirtualKeyCode;
use amethyst::ui::{Anchor, TtfFormat, UiText, UiTransform};

pub struct Paused;
impl<'a, 'b> State<PausableGameData<'a, 'b>, StateEvent> for Paused {
    fn on_start(&mut self, data: StateData<PausableGameData>) {
        data.world.register::<PauseScreenFlag>();
        create_pause_ui(data.world);
    }

    fn handle_event(
        &mut self,
        data: StateData<PausableGameData>,
        event: StateEvent,
    ) -> Trans<PausableGameData<'a, 'b>, StateEvent> {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) {
                Trans::Quit
            } else if is_key_down(&event, VirtualKeyCode::Space) {
                destroy_pause_ui(data.world);
                Trans::Pop
            } else {
                Trans::None
            }
        } else {
            Trans::None
        }
    }

    fn update(
        &mut self,
        data: StateData<PausableGameData>,
    ) -> Trans<PausableGameData<'a, 'b>, StateEvent> {
        data.data.update(&data.world, false);
        Trans::None
    }
}

#[derive(Default)]
struct PauseScreenFlag;

impl Component for PauseScreenFlag {
    type Storage = NullStorage<Self>;
}

fn create_pause_ui(world: &mut World) {
    let font = world.read_resource::<Loader>().load(
        "texture/Merriweather-Regular.ttf",
        TtfFormat,
        Default::default(),
        (),
        &world.read_resource(),
    );

    let transform = UiTransform::new(
        "paused".to_string(),
        Anchor::TopMiddle,
        0.,
        -50.,
        1.,
        200.,
        50.,
        0,
    );

    world
        .create_entity()
        .with(PauseScreenFlag)
        .with(transform)
        .with(UiText::new(
            font.clone(),
            "PAUSED".to_string(),
            [1., 1., 1., 1.],
            50.,
        ))
        .build();
}

fn destroy_pause_ui(world: &mut World) {
    for (ent, _) in (&world.entities(), &world.read_storage::<PauseScreenFlag>()).join() {
        world.entities().delete(ent).unwrap();
    }
}
