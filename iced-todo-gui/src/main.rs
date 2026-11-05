use iced::widget::{Column, TextInput, button, column, row, text};
use iced::{Center, Length};

pub fn main() -> iced::Result {
    iced::run(App::update, App::view)
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
        let mut task_list = Column::new().spacing(8);

        if self.tasks.is_empty() {
            task_list = task_list.push(text("No tasks yet. Add one above!").size(14));
        }

        for task in &self.tasks {
            task_list = task_list.push(row![
                text(&task.text),
                button("Remove")
                    .on_press(Message::RemoveTask(task.id))
                    .width(Length::Fixed(80.0)),
            ]);
        }

        column![
            text("Todo").size(40),
            row![
                TextInput::new("What needs to be done?", &self.input)
                    .on_input(Message::InputChanged)
                    .on_submit(Message::AddTask),
                button("Add").on_press(Message::AddTask),
            ],
            task_list,
        ]
        .width(Length::Fill)
        .align_x(Center)
        .padding(20)
    }
}
