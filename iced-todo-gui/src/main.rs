use iced::widget::{Column, button, column, text};
use iced::{Center, Length};

pub fn main() -> iced::Result {
    iced::run(App::update, App::view)
}

#[derive(Default)]
struct App {
    greet_count: u32,
}

#[derive(Debug, Clone)]
enum Message {
    Greet,
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::Greet => {
                self.greet_count += 1;
            }
        }
    }

    fn view(&self) -> Column<'_, Message> {
        let counter_text = format!(
            "Greeted {} time{}",
            self.greet_count,
            if self.greet_count == 1 { "" } else { "s" }
        );

        column![
            text("Hello, World!").size(40),
            text("A basic iced proof of concept for Linux.").size(18),
            button("Greet").on_press(Message::Greet).padding([10, 20]),
            text(counter_text).size(14),
        ]
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Center)
        .padding(20)
    }
}
