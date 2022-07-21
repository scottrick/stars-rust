use specs::{Component, VecStorage};

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Galaxy {
    pub galaxy_size: i32,
    pub num_planets: u32,
}
