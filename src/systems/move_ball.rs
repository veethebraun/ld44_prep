use amethyst::ecs::{System, Read, ReadStorage, WriteStorage, Join};
use amethyst::core::Transform;

use crate::pong::{Ball, ARENA_HEIGHT, ARENA_WIDTH};

pub struct MoveBallSystem;
impl<'a> System<'a> for MoveBallSystem {
    type SystemData = (
        WriteStorage<'a, Transform>,
        ReadStorage<'a, Ball>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut transforms, balls) = data;
        for (trans, ball) in (&mut transforms, &balls).join() {
            trans.translate_x(ball.vel_x);
            trans.translate_y(ball.vel_y);
        }
    }
}