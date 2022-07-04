use iced::{
    canvas::{self, Cursor, Path, Stroke},
    executor, time, window, Application, Canvas, Color, Command, Element, Length, Point, Rectangle,
    Size, Subscription, Vector,
};
use rand::Rng;
use std::time::Instant;

use crate::planet::Planet;

fn create_planets() -> Vec<Planet> {
    let mut rng = rand::thread_rng();
    (0..10)
        .map(|i| {
            let new_x: f32 = rng.gen();
            let new_y: f32 = rng.gen();
            Planet {
                pos_x: new_x * 256.0,
                pos_y: new_y * 256.0,
                name: format!("planet{i}"),
            }
        })
        .collect()
}

#[derive(Debug)]
struct GalaxyState {
    planets_cache: canvas::Cache,
    planets: Vec<Planet>,
}

impl GalaxyState {
    pub fn new() -> GalaxyState {
        let (width, height) = window::Settings::default().size;

        GalaxyState {
            planets_cache: Default::default(),
            planets: create_planets(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Tick(Instant),
}

pub struct Galaxy {
    state: GalaxyState,
}

impl Application for Galaxy {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Galaxy {
                state: GalaxyState::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Galaxy")
    }

    fn background_color(&self) -> Color {
        Color::BLACK
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Tick(instant) => {}
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        time::every(std::time::Duration::from_millis(10)).map(|instant| Message::Tick(instant))
    }

    fn view(&mut self) -> Element<Message> {
        Canvas::new(&mut self.state)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

impl<Message> canvas::Program<Message> for GalaxyState {
    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<canvas::Geometry> {
        let planets = self.planets_cache.draw(bounds.size(), |frame| {
            /*
            let stars = Path::new(|path| {
                for (p, size) in &self.stars {
                    path.rectangle(*p, Size::new(*size, *size));
                }
            });

            frame.translate(frame.center() - Point::ORIGIN);
            frame.fill(&stars, Color::WHITE);
            */
        });
        vec![planets]
    }
}
