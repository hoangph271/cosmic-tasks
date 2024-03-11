use done_core::models::list::List;
use done_core::models::priority::Priority;
use done_core::models::status::Status;
use done_core::models::task::Task;
use cosmic::iced::{Alignment, Color, Length, Subscription};
use cosmic::iced_widget::row;
use cosmic::{cosmic_theme, theme, widget, Element};

pub struct Content {
    list: Option<List>,
    tasks: Vec<Task>,
    input: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    List(List),
    Rename(String, String),
    Delete(String),
    Select(Task),
    Complete(String, bool),
    SetItems(Vec<Task>),
    ItemDown,
    ItemUp,
    Input(String),
    AddTask,
    SetPriority(String, Priority),
}

pub enum Command {
    GetTasks(String),
    DisplayTask(Task),
    UpdateTask(Task),
    Delete(String),
    CreateTask(Task),
}

impl Content {
    pub fn new() -> Self {
        Self {
            list: None,
            tasks: Vec::new(),
            input: String::new(),
        }
    }

    pub fn list_view(&self) -> Element<Message> {
        let cosmic_theme::Spacing { space_xxs, .. } = theme::active().cosmic().spacing;

        let items = self
            .tasks
            .iter()
            .map(|item| {
                let row = widget::row::with_children(vec![
                    widget::checkbox("", item.status == Status::Completed, |value| {
                        Message::Complete(item.id.clone(), value)
                    })
                    .into(),
                    widget::text(item.title.clone()).width(Length::Fill).into(),
                    widget::button::icon(
                        widget::icon::from_name("user-trash-full-symbolic")
                            .size(16)
                            .handle(),
                    )
                    .on_press(Message::Delete(item.id.clone()))
                    .into(),
                ])
                .align_items(Alignment::Center)
                .spacing(space_xxs);
                widget::button(row)
                    .width(Length::Fill)
                    .height(Length::Shrink)
                    .padding(space_xxs)
                    .style(button_style(false, true))
                    .on_press(Message::Select(item.clone()))
                    .into()
            })
            .collect();
        widget::container(widget::column::with_children(vec![
            widget::column::with_children(items)
                .spacing(space_xxs)
                .height(Length::Fill)
                .into(),
            row(vec![
                widget::text_input("Add new task", &self.input)
                    .on_input(|input| Message::Input(input))
                    .on_submit(Message::AddTask)
                    .width(Length::Fill)
                    .into(),
                widget::button::icon(
                    widget::icon::from_name("list-add-symbolic")
                        .size(16)
                        .handle(),
                )
                .on_press(Message::AddTask)
                .into(),
            ])
            .spacing(space_xxs)
            .align_items(Alignment::Center)
            .into(),
        ]))
        .height(Length::Fill)
        .width(Length::Fill)
        .padding([0, space_xxs, 0, space_xxs])
        .into()
    }

    pub fn update(&mut self, message: Message) -> Vec<Command> {
        let mut commands = Vec::new();
        match message {
            Message::List(list) => {
                self.list = Some(list.clone());
                commands.push(Command::GetTasks(list.id));
            }
            Message::ItemDown => {}
            Message::ItemUp => {}
            Message::Delete(id) => {
                commands.push(Command::Delete(id.clone()));
                self.tasks.retain(|t| t.id != id);
            }
            Message::SetItems(tasks) => self.tasks = tasks,
            Message::Select(task) => {
                commands.push(Command::DisplayTask(task));
            }
            Message::Complete(id, complete) => {
                let task = self.tasks.iter_mut().find(|t| t.id == id);
                if let Some(task) = task {
                    task.status = if complete {
                        Status::Completed
                    } else {
                        Status::NotStarted
                    };
                    commands.push(Command::UpdateTask(task.clone()));
                }
            }
            Message::Input(input) => self.input = input,
            Message::AddTask => {
                if let Some(list) = &self.list {
                    let task = Task::new(self.input.clone(), list.id.clone());
                    commands.push(Command::CreateTask(task.clone()));
                    self.tasks.push(task);
                    self.input.clear();
                }
            }
            Message::SetPriority(id, priority) => {
                let task = self.tasks.iter_mut().find(|t| t.id == id);
                if let Some(task) = task {
                    task.priority = priority;
                    commands.push(Command::UpdateTask(task.clone()));
                }
            }
            Message::Rename(id, title) => {
                let task = self.tasks.iter_mut().find(|t| t.id == id);
                if let Some(task) = task {
                    task.title = title;
                    commands.push(Command::UpdateTask(task.clone()));
                }
            }
        }
        commands
    }

    pub fn view(&self) -> Element<Message> {
        if self.tasks.is_empty() {
            return widget::container(widget::text("No items"))
                .height(Length::Fill)
                .width(Length::Fill)
                .into();
        }

        widget::container(widget::column::with_children(vec![self.list_view()]))
            .height(Length::Fill)
            .width(Length::Fill)
            .into()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }
}

fn button_appearance(
    theme: &theme::Theme,
    selected: bool,
    focused: bool,
    accent: bool,
    hovered: bool,
) -> widget::button::Appearance {
    let cosmic = theme.cosmic();
    let mut appearance = widget::button::Appearance::new();
    if selected {
        if accent {
            appearance.background = Some(Color::from(cosmic.accent_color()).into());
            appearance.icon_color = Some(Color::from(cosmic.on_accent_color()));
            appearance.text_color = Some(Color::from(cosmic.on_accent_color()));
        } else {
            appearance.background = Some(Color::from(cosmic.bg_component_color()).into());
        }
    }
    if hovered {
        appearance.background = Some(Color::from(cosmic.button_bg_color()).into());
        appearance.icon_color = Some(Color::from(cosmic.on_bg_color()));
        appearance.text_color = Some(Color::from(cosmic.on_bg_color()));
    }

    if focused && accent {
        appearance.outline_width = 1.0;
        appearance.outline_color = Color::from(cosmic.accent_color());
        appearance.border_width = 2.0;
        appearance.border_color = Color::TRANSPARENT;
    }
    appearance.border_radius = cosmic.radius_s().into();
    appearance
}

fn button_style(selected: bool, accent: bool) -> theme::Button {
    //TODO: move to libcosmic?
    theme::Button::Custom {
        active: Box::new(move |focused, theme| {
            button_appearance(theme, selected, focused, accent, false)
        }),
        disabled: Box::new(move |theme| button_appearance(theme, selected, false, accent, false)),
        hovered: Box::new(move |focused, theme| {
            button_appearance(theme, selected, focused, accent, true)
        }),
        pressed: Box::new(move |focused, theme| {
            button_appearance(theme, selected, focused, accent, false)
        }),
    }
}