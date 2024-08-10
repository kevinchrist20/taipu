use iced::event::{self, Event};
use iced::keyboard;
use iced::widget::canvas::{Frame, Geometry, Path};
use iced::widget::{canvas, Canvas, Column, Container, Text, TextInput};
use iced::{
    executor, mouse, Application, Color, Command, Font, Point, Settings, Subscription, Theme,
};
use std::fmt::Debug;
use std::{thread, time};

const DEFAULT_KEY_COLOR: Color = Color::from_rgb(0.5, 0.5, 1.0);
const PRESSED_KEY_COLOR: Color = Color::from_rgb(1.0, 0.0, 0.0);

fn main() -> iced::Result {
    TaipuApp::run(Settings::default())
}

enum KeyType {
    Number(char),
    Letter(char),
    Special(String)
}

struct KeyPress {
    key_type: KeyType,
    pressed: bool,
    color: Color,
    width: f32,
}

struct TaipuApp {
    layout: Vec<Vec<KeyPress>>,
    lesson_text: String,
    input_text: String,
    shift_pressed: bool,
    caps_lock: bool,
}

#[derive(Debug, Clone)]
enum Message {
    KeyPressed(char),
    KeyReleased,
    InputChanged(String),
    Nothing,
}

impl Application for TaipuApp {
    type Executor = executor::Default;

    type Message = Message;

    type Theme = Theme;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let layout = vec![
            // Number row
            vec!["`","1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "-", "=", "Backspace"]
                .into_iter()
                .map(create_key)
                .collect(),
            // QWERTY row
            vec!["q", "w", "e", "r", "t", "y", "u", "i", "o", "p", "[", "]", "\\"]
                .into_iter()
                .map(create_key)
                .collect(),
            // Home row
            vec!["Caps", "a", "s", "d", "f", "g", "h", "j", "k", "l", ";", "'", "Enter"]
                .into_iter()
                .map(create_key)
                .collect(),
            // Bottom row
            vec!["Shift", "z", "x", "c", "v", "b", "n", "m", ",", ".", "/", "Shift"]
                .into_iter()
                .map(create_key)
                .collect(),
            // Space bar row
            vec!["Ctrl", "Win", "Alt", "Space", "Alt", "Fn", "Ctrl"]
                .into_iter()
                .map(create_key)
                .collect(),
        ];

        (
            Self { 
                layout,
                lesson_text: String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed non risus. Suspendisse lectus tortor, dignissim sit amet, adipiscing nec, ultricies sed, dolor. Cras elementum ultrices diam. Maecenas ligula massa, varius a, semper congue, euismod non, mi."),
                input_text: String::new(),
                shift_pressed: false,
                caps_lock: false,
            }, 
        Command::none()
    )
    }

    fn title(&self) -> String {
        String::from("Taipu")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::KeyPressed(character) => {
                // if let Some(key_press) = self.layout.iter_mut().find(|key| key.key == character) {
                //     key_press.pressed = true;
                //     key_press.color = PRESSED_KEY_COLOR;

                //     self.input_text.push(character);
                // }
            }

            Message::KeyReleased => {
                // for key_press in self.layout.iter_mut() {
                //     key_press.pressed = false;
                //     key_press.color = DEFAULT_KEY_COLOR;
                // }
            }

            Message::InputChanged(new_value) => {
                self.input_text = new_value;
            }

            _ => {}
        }

        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        let keyboard_canvas = Canvas::new(self)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill);

        let lesson_text = Text::new(&self.lesson_text).size(20);
        let typing_area = TextInput::new(
            "Type here...",
            &self.input_text,
        )
        // .on_input(Message::InputChanged)
        .padding(10)
        .size(20);

        let keyboard_area = Column::new().push(keyboard_canvas).padding(20);
        
        Container::new(
            Column::new()
                .push(lesson_text)
                .push(typing_area)
                .push(keyboard_area)
        )
        .padding(20)
        .into()
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

impl<Message> canvas::Program<Message> for TaipuApp {
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

        let total_width = bounds.width;
        let total_height = bounds.height * 0.4;
        let start_y = bounds.height - total_height;

        let row_height = total_height / self.layout.len() as f32;
        let unit_width = total_width / 15.0;

        for (row_index, row) in self.layout.iter().enumerate() {
            let mut x = 0.0;
            let y = start_y + row_index as f32 * row_height;

            for key in row {
                let key_width = key.width * unit_width;
                let key_height = row_height * 0.9;

                let rectangle = Path::rectangle(
                    Point::new(x, y),
                    iced::Size::new(key_width, key_height),
                );

                frame.fill(&rectangle, key.color);

                // Draw the key label
                let label = match &key.key_type {
                    KeyType::Letter(c) | KeyType::Number(c) =>
                     c.to_string(),
                    KeyType::Special(s) => s.clone(),
                };

                frame.fill_text(
                        canvas::Text {
                        content: label,
                        position: Point::new(x + key_width / 2.0, y + key_height / 2.0),
                        color: Color::WHITE,
                        font: Font::MONOSPACE,
                        size: iced::Pixels(14.0),
                        horizontal_alignment: iced::alignment::Horizontal::Center,
                        vertical_alignment: iced::alignment::Vertical::Center,
                        ..Default::default()
                    }
                );

                let hr_padding = 5.0;
                x += key_width + hr_padding;
            }
        }

        vec![frame.into_geometry()]
    }
}

fn create_key(key: &str) -> KeyPress {
    let (key_type, width) = match key {
        "Enter" | "Caps" => (KeyType::Special(key.to_string()), 1.5),
        "Backspace" => (KeyType::Special(key.to_string()), 1.2),
        "Shift" => (KeyType::Special(key.to_string()), 2.2),
        "Space" => (KeyType::Special(key.to_string()), 6.0),
        "Ctrl" | "Win" | "Alt" | "Fn" => (KeyType::Special(key.to_string()), 1.0),
        key if key.len() == 1 => {
            let c = key.chars().next().unwrap();
            if c.is_ascii_digit(){
                (KeyType::Number(c), 1.0)
            } else {
                (KeyType::Letter(c), 1.0)
            }
        }
        _ => (KeyType::Special(key.to_string()), 1.0)
    };

    KeyPress {
        key_type,
        pressed: false,
        color: DEFAULT_KEY_COLOR,
        width,
    }
}
