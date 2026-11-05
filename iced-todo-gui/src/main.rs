use iced::widget::text::Wrapping;
use iced::widget::{Column, TextInput, button, column, container, row, scrollable, text};
use iced::{Center, Length};

pub fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .window(iced::window::Settings {
            size: iced::Size::new(720.0, 540.0),
            min_size: Some(iced::Size::new(480.0, 360.0)),
            ..iced::window::Settings::default()
        })
        .run()
}

#[derive(Default)]
struct App {
    tasks: Vec<Task>,
    input: String,
    next_id: u64,
}

#[derive(Debug, Clone)]
struct Task {
    id: u64,
    text: String,
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    AddTask,
    RemoveTask(u64),
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::InputChanged(value) => {
                self.input = value;
            }
            Message::AddTask => {
                let task_text = self.input.trim().to_string();

                if !task_text.is_empty() {
                    self.tasks.push(Task {
                        id: self.next_id,
                        text: task_text,
                    });
                    self.next_id += 1;
                    self.input.clear();
                }
            }
            Message::RemoveTask(id) => {
                self.tasks.retain(|task| task.id != id);
            }
        }
    }

    fn view(&self) -> Column<'_, Message> {
        let mut task_list = Column::new().width(Length::Fill).align_x(Center).spacing(8);

        if self.tasks.is_empty() {
            task_list = task_list.push(text("No tasks yet. Add one above!").size(14));
        }

        for task in &self.tasks {
            task_list = task_list.push(
                container(
                    row![
                        text(&task.text)
                            .width(Length::Fill)
                            .wrapping(Wrapping::WordOrGlyph),
                        button("Remove")
                            .on_press(Message::RemoveTask(task.id))
                            .width(Length::Fixed(80.0)),
                    ]
                    .width(Length::Fill)
                    .spacing(8),
                )
                .width(Length::Fill)
                .padding(10)
                .style(container::rounded_box),
            );
        }

        column![
            text("Todo").size(40),
            row![
                TextInput::new("What needs to be done?", &self.input)
                    .on_input(Message::InputChanged)
                    .on_submit(Message::AddTask),
                button("Add").on_press(Message::AddTask),
            ]
            .padding(10),
            scrollable(task_list)
                .width(Length::Fill)
                .height(Length::Fill)
                .spacing(10),
        ]
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Center)
        .padding(20)
    }
}
