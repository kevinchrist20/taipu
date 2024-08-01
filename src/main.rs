use iced::event::{self, Event};
use iced::keyboard;
use iced::widget::canvas::{Frame, Geometry, Path};
use iced::widget::{canvas, Canvas, Column};
use iced::{
    executor, mouse, Application, Color, Command, Font, Point, Settings, Subscription, Theme,
};
use std::fmt::Debug;
use std::{thread, time};

const DEFAULT_KEY_COLOR: Color = Color::from_rgb(0.5, 0.5, 1.0);
const PRESSED_KEY_COLOR: Color = Color::from_rgb(1.0, 0.0, 0.0);

fn main() -> iced::Result {
    AppKeyboard::run(Settings::default())
}

struct KeyPress {
    key: char,
    pressed: bool,
    color: Color,
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
        let keys: Vec<KeyPress> = "qwertyuiop\
                                    asdfghjkl;\
                                    zxcvbnm,./"
            .chars()
            .map(|c| KeyPress {
                key: c,
                pressed: false,
                color: DEFAULT_KEY_COLOR,
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
                    key_press.color = PRESSED_KEY_COLOR;
                }
            }

            Message::KeyReleased => {
                for key_press in self.layout.iter_mut() {
                    key_press.pressed = false;
                    key_press.color = DEFAULT_KEY_COLOR;
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
                    keyboard::Event::KeyReleased { .. } => {
                        thread::sleep(time::Duration::from_millis(100));
                        Message::KeyReleased
                    }
                    keyboard::Event::KeyPressed { text, .. } => {
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

        let num_of_rows = 3;
        let num_of_cols = 10;

        let total_keyboard_width = bounds.width;
        let total_keyboard_height = bounds.height * 0.3;

        let key_width = total_keyboard_width / (num_of_cols as f32) * 0.9;
        let hr_padding = total_keyboard_width / (num_of_cols as f32) * 0.1;
        let key_height = total_keyboard_height / (num_of_rows as f32) * 0.9;
        let vertical_padding = total_keyboard_height / (num_of_rows as f32) * 0.1;

        let start_y_pos = bounds.height - total_keyboard_height;

        for (i, key) in self.layout.iter().enumerate() {
            let row = i / num_of_cols;
            let col = i % num_of_cols;

            let x = col as f32 * (key_width + hr_padding) + hr_padding / 2.0;
            let y =
                start_y_pos + row as f32 * (key_height + vertical_padding) + vertical_padding / 2.0;

            let rectangle =
                Path::rectangle(Point::new(x, y), iced::Size::new(key_width, key_height));

            frame.fill(&rectangle, key.color);

            // Draw the key label
            frame.fill_text(canvas::Text {
                content: key.key.to_string(),
                position: Point::new(x + key_width / 2.0, y + key_height / 2.0),
                color: Color::WHITE,
                font: Font::MONOSPACE,
                size: iced::Pixels(24.0),
                horizontal_alignment: iced::alignment::Horizontal::Center,
                vertical_alignment: iced::alignment::Vertical::Center,
                ..Default::default()
            });
        }

        vec![frame.into_geometry()]
    }
}
