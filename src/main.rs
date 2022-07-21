use components::{galaxy::Galaxy, named::Named, position::Position};
use rand::Rng;
use specs::{Builder, DispatcherBuilder, RunNow, World, WorldExt};
use systems::{hello::Hello, movement::Movement};

pub mod components;
pub mod planet;
pub mod planets;
pub mod solar_system;
pub mod systems;

// struct Hello;

// impl Sandbox for Hello {
//     type Message = ();

//     fn new() -> Hello {
//         Hello
//     }

//     fn title(&self) -> String {
//         String::from("A cool application")
//     }

//     fn update(&mut self, _message: Self::Message) {
//         // This application has no interactions
//     }

//     fn view(&mut self) -> Element<Self::Message> {
//         Text::new("Hello, world!").into()
//     }
// }

// pub fn main() -> iced::Result {
pub fn main() {
    let mut rng = rand::thread_rng();

    let mut world = World::new();
    world.register::<Position>();
    world.register::<Named>();
    world.register::<Galaxy>();

    let galaxy_size: i32 = 1024;
    let num_planets: u32 = 2;

    let galaxy = world
        .create_entity()
        .with(Galaxy {
            galaxy_size,
            num_planets,
        })
        .build();

    (0..num_planets).for_each(|i| {
        let new_x: i32 = rng.gen_range(0..galaxy_size);
        let new_y: i32 = rng.gen_range(0..galaxy_size);

        let planet = world
            .create_entity()
            .with(Position { x: new_x, y: new_y })
            .build();
    });

    // let mut hello_system = Hello;
    // hello_system.run_now(&world);
    // world.maintain();

    // let mut movement_system = Movement;
    // movement_system.run_now(&world);
    // world.maintain();

    // hello_system.run_now(&world);
    // world.maintain();

    let mut dispatcher = DispatcherBuilder::new()
        .with(Hello, "initial-hello", &[])
        .with(Movement, "movement", &["initial-hello"])
        .with(Hello, "second-hello", &["movement"])
        .build();

    dispatcher.dispatch(&mut world);

    // Galaxy::run(Settings {
    //     antialiasing: true,
    //     ..Settings::default()
    // })

    // SolarSystem::run(Settings {
    //     antialiasing: true,
    //     ..Settings::default()
    // })
}
