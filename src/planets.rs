use iced::{
    canvas::{self, Cursor, Path, Stroke},
    executor, time, window, Application, Canvas, Color, Command, Element, Length, Point, Rectangle,
    Size, Subscription, Vector,
};
use rand::Rng;
use std::time::Instant;

use crate::planet::Planet;

fn create_planets(galaxy_size: f32, num_planets: u32) -> Vec<Planet> {
    let mut rng = rand::thread_rng();
    (0..num_planets)
        .map(|i| {
            let new_x: f32 = rng.gen();
            let new_y: f32 = rng.gen();
            Planet {
                pos_x: new_x * galaxy_size,
                pos_y: new_y * galaxy_size,
                name: format!("planet{i}"),
            }
        })
        .collect()
}

#[derive(Debug)]
struct GalaxyState {
    galaxy_size: f32,
    planets_cache: canvas::Cache,
    planets: Vec<Planet>,
}

impl GalaxyState {
    pub fn new(galaxy_size: f32, num_planets: u32) -> GalaxyState {
        let (width, height) = window::Settings::default().size;

        GalaxyState {
            galaxy_size,
            planets_cache: Default::default(),
            planets: create_planets(galaxy_size, num_planets),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Dummy,
    // Tick(Instant),
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
                state: GalaxyState::new(256.0, 100),
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
            Message::Dummy => {
                println!("Received dummy message.");
            } // Message::Tick(instant) => {}
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
        // time::every(std::time::Duration::from_millis(10)).map(|instant| Message::Tick(instant))
    }

    fn mode(&self) -> window::Mode {
        window::Mode::Windowed
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
        println!("draw!");
        let planets = self.planets_cache.draw(bounds.size(), |frame| {
            let planets = Path::new(|path| {
                for p in &self.planets {
                    path.circle(
                        Point {
                            x: p.pos_x,
                            y: p.pos_y,
                        },
                        4.0,
                    );
                }
            });

            frame.translate(frame.center() - Point::ORIGIN);
            frame.fill(&planets, Color::WHITE);

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
