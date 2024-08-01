use iced::event::{self, Event};
use iced::keyboard;
use iced::widget::canvas::{Frame, Geometry, Path};
use iced::widget::{canvas, Canvas, Column};
use iced::{
    executor, mouse, Application, Color, Command, Font, Point, Settings, Subscription, Theme,
};
use std::fmt::Debug;

fn main() -> iced::Result {
    AppKeyboard::run(Settings::default())
}

struct KeyPress {
    key: char,
    pressed: bool,
}

struct AppKeyboard {
    layout: Vec<KeyPress>,
}

#[derive(Debug, Clone)]
enum Message {
    KeyPressed(char),
    KeyReleased,
    Nothing,
}

impl Application for AppKeyboard {
    type Executor = executor::Default;

    type Message = Message;

    type Theme = Theme;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let keys: Vec<KeyPress> = "qwertyuiopasdfghjkl;zxcvbnm,./"
            .chars()
            .map(|c| KeyPress {
                key: c,
                pressed: false,
            })
            .collect();

        (Self { layout: keys }, Command::none())
    }

    fn title(&self) -> String {
        String::from("Taipu")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::KeyPressed(character) => {
                if let Some(key_press) = self.layout.iter_mut().find(|key| key.key == character) {
                    key_press.pressed = true;
                }
            }

            // TODO: Better
            Message::KeyReleased => {
                for key_press in self.layout.iter_mut() {
                    key_press.pressed = false;
                }
            }

            _ => {}
        }

        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        let keyboard_canvas = Canvas::new(self)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill);

        Column::new().push(keyboard_canvas).padding(20).into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        event::listen().map(|message| {
            if let Event::Keyboard(key_event) = message {
                match key_event {
                    keyboard::Event::KeyReleased { key, .. } => {
                        println!("Released{:?}", key);
                        Message::KeyReleased
                    }
                    keyboard::Event::KeyPressed { key, text, .. } => {
                        println!("Pressed{:?}", text);
                        Message::KeyPressed(text.unwrap().chars().next().unwrap())
                    }
                    _ => Message::Nothing,
                }
            } else {
                Message::Nothing
            }
        })
    }
}

impl<Message> canvas::Program<Message> for AppKeyboard {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &Theme,
        bounds: iced::Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());

        let key_width = 40.0;
        let key_height = 40.0;
        let padding = 10.0;

        for (i, key) in self.layout.iter().enumerate() {
            let x = (i as f32 % 10.0) * (key_width + padding);
            let y = (i as f32 / 10.0).floor() * (key_height + padding);

            let color = if key.pressed {
                Color::from_rgb(255.0, 0.0, 0.0)
                // Color::from_rgb(0.5, 0.5, 1.0)
            } else {
                Color::WHITE
            };

            frame.fill_rectangle(
                Point::new(x, y),
                iced::Size::new(key_width, key_height),
                color,
            );

            let rectangle =
                Path::rectangle(Point::new(x, y), iced::Size::new(key_width, key_height));

            frame.fill(&rectangle, Color::BLACK);

            // Draw the key label
            frame.fill_text(canvas::Text {
                content: key.key.to_string(),
                position: Point::new(x + key_width / 2.0, y + key_height / 2.0),
                color: Color::WHITE,
                font: Font::MONOSPACE,
                size: iced::Pixels(20.0),
                horizontal_alignment: iced::alignment::Horizontal::Center,
                vertical_alignment: iced::alignment::Vertical::Center,
                ..Default::default()
            });
        }

        vec![frame.into_geometry()]
    }
}
