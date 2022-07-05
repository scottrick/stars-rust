use iced::{Application, Settings};
use planets::Galaxy;
use solar_system::SolarSystem;

use planet::Planet;
use rand::Rng;

pub mod planet;
pub mod planets;
pub mod solar_system;

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

pub fn main() -> iced::Result {
    Galaxy::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })

    // SolarSystem::run(Settings {
    //     antialiasing: true,
    //     ..Settings::default()
    // })
}
