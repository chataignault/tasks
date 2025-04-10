use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Offset, Rect},
    style::Stylize,
    text::Line,
    widgets::Widget,
    Frame,
};
use serde::Serialize;

use crate::base::{Status, TodoItem};

#[derive(Serialize)]
pub struct TodoForm {
    #[serde(skip)]
    pub focus: Focus,
    name: StringField,
    description: StringField,
}

impl Default for TodoForm {
    fn default() -> Self {
        Self {
            focus: Focus::Name,
            name: StringField::new("Name"),
            description: StringField::new("Description"),
        }
    }
}

impl TodoForm {
    // Handle focus navigation or pass the event to the focused field.
    pub fn on_key_press(&mut self, event: KeyEvent) {
        if event.kind != KeyEventKind::Press {
            return;
        }
        match event.code {
            KeyCode::Tab => self.focus = self.focus.next(),
            _ => match self.focus {
                Focus::Name => self.name.on_key_press(event),
                Focus::Description => self.description.on_key_press(event),
            },
        }
    }

    /// Render the form with the current focus.
    ///
    /// The cursor is placed at the end of the focused field.
    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        let [name_area, description_area] =
            Layout::vertical(Constraint::from_lengths([1, 1])).areas(area);

        self.name.render(name_area, buf);
        self.description.render(description_area, buf);
    }

    pub fn extract(&mut self) -> TodoItem {
        let todo = TodoItem {
            todo: self.name.value.to_string(),
            info: self.description.value.to_string(),
            status: Status::Todo,
        };
        self.name.value = "".to_string();
        self.description.value = "".to_string();
        todo
    }
}

#[derive(Default, PartialEq, Eq)]
pub enum Focus {
    #[default]
    Name,
    Description,
}

impl Focus {
    // Round-robin focus order.
    const fn next(&self) -> Self {
        match self {
            Self::Name => Self::Description,
            Self::Description => Self::Name,
        }
    }
}

/// A new-type representing a string field with a label.
#[derive(Debug, Serialize)]
struct StringField {
    #[serde(skip)]
    label: &'static str,
    value: String,
}

impl StringField {
    const fn new(label: &'static str) -> Self {
        Self {
            label,
            value: String::new(),
        }
    }

    /// Handle input events for the string input.
    fn on_key_press(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Char(c) => self.value.push(c),
            KeyCode::Backspace => {
                self.value.pop();
            }
            _ => {}
        }
    }

    fn cursor_offset(&self) -> Offset {
        let x = (self.label.len() + self.value.len() + 2) as i32;
        Offset { x: x, y: 0 }
    }
}

impl Widget for &StringField {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let constraints = [
            Constraint::Length(self.label.len() as u16 + 5),
            Constraint::Fill(1),
        ];
        let [label_area, value_area] = Layout::horizontal(constraints).areas(area);
        let label = Line::from_iter(["   ", self.label, ": "]).bold();
        label.render(label_area, buf);
        self.value.clone().render(value_area, buf);
    }
}
