use specs::{ReadStorage, System};

use crate::components::position::Position;

pub struct Hello;

impl<'a> System<'a> for Hello {
    type SystemData = ReadStorage<'a, Position>;

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;

        for position in data.join() {
            println!("Hello: {position:?}");
        }
    }
}
