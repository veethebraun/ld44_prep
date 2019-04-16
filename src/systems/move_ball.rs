use amethyst::ecs::{System, Read, ReadStorage, WriteStorage, Join};
use amethyst::core::Transform;

use crate::pong::{Ball, ARENA_HEIGHT, ARENA_WIDTH};

pub struct MoveBallSystem;
impl<'a> System<'a> for MoveBallSystem {
    type SystemData = (
        WriteStorage<'a, Transform>,
        WriteStorage<'a, Ball>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut transforms, mut balls) = data;
        for (trans, ball) in (&mut transforms, &mut balls).join() {
            trans.translate_x(ball.vel_x);
            trans.translate_y(ball.vel_y);

            let translation = trans.translation();
            if translation[0] <= 0. || translation[0] >= ARENA_WIDTH {
                ball.vel_x *= -1.;
            }
            if translation[1] <= 0. || translation[1] >= ARENA_HEIGHT {
                ball.vel_y *= -1.;
            }
        }
    }
}