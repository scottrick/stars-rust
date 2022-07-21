use specs::{System, WriteStorage};

use crate::components::position::Position;

pub struct Movement;

impl<'a> System<'a> for Movement {
    type SystemData = WriteStorage<'a, Position>;

    fn run(&mut self, mut position: Self::SystemData) {
        use specs::Join;
        println!(" --> movement");

        for pos in (&mut position).join() {
            pos.x += 1;
            pos.y += 1;
        }
    }
}
