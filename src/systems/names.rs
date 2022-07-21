use specs::{ReadStorage, System};

use crate::components::named::Named;

pub struct Names;

impl<'a> System<'a> for Names {
    type SystemData = ReadStorage<'a, Named>;

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;

        for name in data.join() {
            println!("Hello: {name:?}");
        }
    }
}
