use iced::{
    canvas::{self, Cursor, Path, Stroke},
    executor, time, window, Application, Canvas, Color, Command, Element, Length, Point, Rectangle,
    Settings, Size, Subscription, Vector,
};

use std::time::Instant;

use planet::Planet;
use rand::Rng;

pub mod planet;

// pub fn main() -> iced::Result {
//     Hello::run(Settings::default())
// }

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
    // let mut rng = rand::thread_rng();
    // for i in 1..11 {
    //     let new_x: f64 = rng.gen();
    //     let new_y: f64 = rng.gen();
    //     let new_planet = Planet {
    //         pos_x: new_x * 256.0f64,
    //         pos_y: new_y * 256.0f64,
    //         name: format!("planet{i}"),
    //     };

    //     println!("{new_planet:?}");
    // }

    SolarSystem::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}

struct SolarSystem {
    state: State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Tick(Instant),
}

impl Application for SolarSystem {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            SolarSystem {
                state: State::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Solar system - Iced")
    }

    fn background_color(&self) -> Color {
        Color::BLACK
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Tick(instant) => {
                self.state.update(instant);
            }
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

#[derive(Debug)]
struct State {
    space_cache: canvas::Cache,
    system_cache: canvas::Cache,
    start: Instant,
    now: Instant,
    stars: Vec<(Point, f32)>,
}

impl State {
    const SUN_RADIUS: f32 = 70.0;
    const ORBIT_RADIUS: f32 = 150.0;
    const EARTH_RADIUS: f32 = 12.0;
    const MOON_RADIUS: f32 = 4.0;
    const MOON_DISTANCE: f32 = 28.0;

    pub fn new() -> State {
        let now = Instant::now();
        let (width, height) = window::Settings::default().size;

        State {
            space_cache: Default::default(),
            system_cache: Default::default(),
            start: now,
            now,
            stars: Self::generate_stars(width, height),
        }
    }

    pub fn update(&mut self, now: Instant) {
        self.now = now;
        self.system_cache.clear();
    }

    fn generate_stars(width: u32, height: u32) -> Vec<(Point, f32)> {
        let mut rng = rand::thread_rng();

        (0..100)
            .map(|_| {
                (
                    Point::new(
                        rng.gen_range((-(width as f32) / 2.0)..(width as f32 / 2.0)),
                        rng.gen_range((-(height as f32) / 2.0)..(height as f32 / 2.0)),
                    ),
                    rng.gen_range(0.5..1.0),
                )
            })
            .collect()
    }
}

impl<Message> canvas::Program<Message> for State {
    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<canvas::Geometry> {
        use std::f32::consts::PI;

        let background = self.space_cache.draw(bounds.size(), |frame| {
            let stars = Path::new(|path| {
                for (p, size) in &self.stars {
                    path.rectangle(*p, Size::new(*size, *size));
                }
            });

            frame.translate(frame.center() - Point::ORIGIN);
            frame.fill(&stars, Color::WHITE);
        });

        let system = self.system_cache.draw(bounds.size(), |frame| {
            let center = frame.center();

            let sun = Path::circle(center, Self::SUN_RADIUS);
            let orbit = Path::circle(center, Self::ORBIT_RADIUS);

            frame.fill(&sun, Color::from_rgb8(0xF9, 0xD7, 0x1C));
            frame.stroke(
                &orbit,
                Stroke {
                    width: 1.0,
                    color: Color::from_rgba8(0, 153, 255, 0.1),
                    line_dash: canvas::LineDash {
                        offset: 0,
                        segments: &[3.0, 6.0],
                    },
                    ..Stroke::default()
                },
            );

            let elapsed = self.now - self.start;
            let rotation = (2.0 * PI / 60.0) * elapsed.as_secs() as f32
                + (2.0 * PI / 60_000.0) * elapsed.subsec_millis() as f32;

            frame.with_save(|frame| {
                frame.translate(Vector::new(center.x, center.y));
                frame.rotate(rotation);
                frame.translate(Vector::new(Self::ORBIT_RADIUS, 0.0));

                let earth = Path::circle(Point::ORIGIN, Self::EARTH_RADIUS);
                let shadow = Path::rectangle(
                    Point::new(0.0, -Self::EARTH_RADIUS),
                    Size::new(Self::EARTH_RADIUS * 4.0, Self::EARTH_RADIUS * 2.0),
                );

                frame.fill(&earth, Color::from_rgb8(0x6B, 0x93, 0xD6));

                frame.with_save(|frame| {
                    frame.rotate(rotation * 10.0);
                    frame.translate(Vector::new(0.0, Self::MOON_DISTANCE));

                    let moon = Path::circle(Point::ORIGIN, Self::MOON_RADIUS);
                    frame.fill(&moon, Color::WHITE);
                });

                frame.fill(
                    &shadow,
                    Color {
                        a: 0.7,
                        ..Color::BLACK
                    },
                );
            });
        });

        vec![background, system]
    }
}
