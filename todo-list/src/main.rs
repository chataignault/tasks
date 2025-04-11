/// A Ratatui example that demonstrates how to create a todo list with selectable items.
///
/// This example runs with the Ratatui library code in the branch that you are currently
/// reading. See the [`latest`] branch for the code which works with the most recent Ratatui
/// release.
///
/// [`latest`]: https://github.com/ratatui/ratatui/tree/latest
use color_eyre::Result;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::{
        palette::tailwind::{BLUE, GREEN, ORANGE, SLATE},
        Color, Modifier, Style, Stylize,
    },
    symbols,
    text::Line,
    widgets::{
        Block, Borders, Clear, HighlightSpacing, List, ListItem, ListState, Padding, Paragraph,
        StatefulWidget, Widget, Wrap,
    },
    DefaultTerminal,
};

mod base;
mod form;
mod popup;
mod utils;
use base::{Status, TodoItem};
use form::{Focus, TodoForm};

const TODO_HEADER_STYLE: Style = Style::new().fg(SLATE.c100).bg(BLUE.c800);
const NORMAL_ROW_BG: Color = SLATE.c950;
const ALT_ROW_BG_COLOR: Color = SLATE.c900;
const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);
const TEXT_FG_COLOR: Color = SLATE.c200;
const COMPLETED_TEXT_FG_COLOR: Color = GREEN.c500;
const IN_PROGRESS_TEXT_FG_COLOR: Color = ORANGE.c300;

const DEFAULT_TOPIC: &str = "general";
const DEFAULT_HISTORY: &str = "history";

pub struct TodoList {
    items: Vec<TodoItem>,
    state: ListState,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::default().run(terminal);
    ratatui::restore();
    app_result
}

/// This struct holds the current state of the app. In particular, it has the `todo_list` field
/// which is a wrapper around `ListState`. Keeping track of the state lets us render the
/// associated widget with its state and have access to features such as natural scrolling.
///
/// Check the event handling at the bottom to see how to change the state on incoming events. Check
/// the drawing logic for items on how to specify the highlighting style for selected items.
struct App {
    should_exit: bool,
    focus_history: bool,
    todo_list: TodoList,
    history_list: TodoList,
    todo_form: TodoForm,
    popup_mode: bool,
}

impl Default for App {
    fn default() -> Self {
        let todos: Vec<TodoItem> =
            utils::load_todo_items((DEFAULT_TOPIC.to_string() + ".json").as_str()).unwrap();
        let history: Vec<TodoItem> =
            utils::load_todo_items((DEFAULT_HISTORY.to_string() + ".json").as_str()).unwrap();
        if todos.len() > 0 {
            Self {
                should_exit: false,
                focus_history: false,
                todo_list: TodoList {
                    items: todos,
                    state: ListState::default(),
                },
                history_list: TodoList {
                    items: history,
                    state: ListState::default(),
                },
                todo_form: TodoForm::default(),
                popup_mode: false,
            }
        } else {
            Self {
                should_exit: false,
                focus_history: false,
                todo_list: TodoList::from_iter([(
                    Status::Todo,
                    "Create a task",
                    "Press 'a' to start writing a task",
                )]),
                history_list: TodoList::from_iter([]),
                todo_form: TodoForm::default(),
                popup_mode: false,
            }
        }
    }
}

impl FromIterator<(Status, &'static str, &'static str)> for TodoList {
    fn from_iter<I: IntoIterator<Item = (Status, &'static str, &'static str)>>(iter: I) -> Self {
        let items = iter
            .into_iter()
            .map(|(status, todo, info)| TodoItem::new(status, todo, info))
            .collect();
        let state = ListState::default();
        Self { items, state }
    }
}

impl App {
    fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while !self.should_exit {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            if let Event::Key(key) = event::read()? {
                if self.popup_mode {
                    if (key.code == KeyCode::Esc) | (key.code == KeyCode::Enter) {
                        self.popup_mode = !self.popup_mode;
                        // get the form values and add it to the todo list
                        let todo = self.todo_form.extract();
                        if key.code == KeyCode::Enter {
                            self.todo_list.items.push(todo);
                        }
                    } else {
                        self.todo_form.on_key_press(key);
                    }
                } else {
                    self.handle_key(key);
                }
            };
        }
        // save the json with updated status
        utils::save_list(
            self.todo_list.items,
            (DEFAULT_TOPIC.to_string() + ".json").as_str(),
        );
        utils::save_list(
            self.history_list.items,
            (DEFAULT_HISTORY.to_string() + ".json").as_str(),
        );
        Ok(())
    }

    fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.should_exit = true,
            KeyCode::Char('h') | KeyCode::Left => self.select_none(),
            KeyCode::Char('j') | KeyCode::Down => self.select_next(),
            KeyCode::Char('k') | KeyCode::Up => self.select_previous(),
            KeyCode::Char('g') | KeyCode::Home => self.select_first(),
            KeyCode::Char('G') | KeyCode::End => self.select_last(),
            KeyCode::Char('l') | KeyCode::Right | KeyCode::Enter => {
                self.toggle_status();
            }
            KeyCode::Char('w') => self.switch_todo_history(),
            KeyCode::Char('f') => self.flush_items(),
            KeyCode::Char('a') => self.add_todo(),
            _ => {}
        }
    }

    fn select_none(&mut self) {
        if self.focus_history {
            self.history_list.state.select(None);
        } else {
            self.todo_list.state.select(None);
        }
    }

    fn select_next(&mut self) {
        if self.focus_history {
            self.history_list.state.select_next();
        } else {
            self.todo_list.state.select_next();
        }
    }
    fn select_previous(&mut self) {
        if self.focus_history {
            self.history_list.state.select_previous();
        } else {
            self.todo_list.state.select_previous();
        }
    }

    fn select_first(&mut self) {
        if self.focus_history {
            self.history_list.state.select_first();
        } else {
            self.todo_list.state.select_first();
        }
    }

    fn select_last(&mut self) {
        self.todo_list.state.select_last();
    }

    /// Changes the status of the selected list item
    fn toggle_status(&mut self) {
        if self.focus_history {
            if let Some(i) = self.history_list.state.selected() {
                self.history_list.items[i].status = match self.history_list.items[i].status {
                    Status::Completed => Status::Todo,
                    Status::Todo => Status::InProgress,
                    Status::InProgress => Status::Completed,
                }
            }
        } else {
            if let Some(i) = self.todo_list.state.selected() {
                self.todo_list.items[i].status = match self.todo_list.items[i].status {
                    Status::Completed => Status::Todo,
                    Status::Todo => Status::InProgress,
                    Status::InProgress => Status::Completed,
                }
            }
        }
    }

    fn switch_todo_history(&mut self) {
        self.focus_history = !self.focus_history;
    }

    fn flush_items(&mut self) {
        let mut done_items = TodoList {
            items: self
                .history_list
                .items
                .iter()
                .filter(|item| item.status == Status::Completed)
                .cloned()
                .collect::<Vec<_>>(),
            state: self.history_list.state.clone(),
        };
        done_items.items.extend(
            self.todo_list
                .items
                .iter()
                .filter(|item| item.status == Status::Completed)
                .cloned()
                .collect::<Vec<_>>(),
        );
        let mut other_items = TodoList {
            items: self
                .history_list
                .items
                .iter()
                .filter(|item| item.status != Status::Completed)
                .cloned()
                .collect(),
            state: self.todo_list.state.clone(),
        };
        other_items.items.extend(
            self.todo_list
                .items
                .iter()
                .filter(|item| item.status != Status::Completed)
                .cloned()
                .collect::<Vec<_>>(),
        );
        self.todo_list = other_items;
        self.history_list = done_items;
    }

    fn add_todo(&mut self) {
        self.popup_mode = !self.popup_mode;
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [header_area, main_area, footer_area] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(area);

        let [list_area, item_area, history_area] = Layout::vertical([
            Constraint::Fill(2),
            Constraint::Fill(1),
            Constraint::Fill(1),
        ])
        .areas(main_area);

        App::render_header(header_area, buf);
        App::render_footer(footer_area, buf);
        self.render_list(list_area, buf);
        self.render_selected_item(item_area, buf);
        self.render_history(history_area, buf);

        // pop up
        if self.popup_mode {
            let block = Block::bordered().on_light_magenta();
            let popup_area = popup::popup_area(area, 60, 20);
            let vertical =
                Layout::vertical([Constraint::Percentage(20), Constraint::Percentage(80)]);
            let [instructions, content] = vertical.areas(popup_area);
            let text = "New task";
            let paragraph = Paragraph::new(text.slow_blink())
                .centered()
                .wrap(Wrap { trim: true });
            Clear.render(popup_area, buf);
            block.render(popup_area, buf);
            paragraph.render(instructions, buf);
            self.todo_form.render(content, buf);
        }
    }
}

/// Rendering logic for the app
impl App {
    fn render_header(area: Rect, buf: &mut Buffer) {
        Paragraph::new("Todo List")
            .bold()
            .centered()
            .render(area, buf);
    }

    fn render_footer(area: Rect, buf: &mut Buffer) {
        Paragraph::new("Use ↓↑ to move, ← to unselect, → to change status, g/G to go top/bottom.")
            .centered()
            .render(area, buf);
    }

    fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Line::raw("TODO List").centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(TODO_HEADER_STYLE)
            .bg(NORMAL_ROW_BG);

        // Iterate through all elements in the `items` and stylize them.
        let items: Vec<ListItem> = self
            .todo_list
            .items
            .iter()
            .enumerate()
            .map(|(i, todo_item)| {
                let color = alternate_colors(i);
                ListItem::from(todo_item).bg(color)
            })
            .collect();

        // Create a List from all list items and highlight the currently selected one
        let list = List::new(items)
            .block(block)
            .highlight_style(SELECTED_STYLE)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        // We need to disambiguate this trait method as both `Widget` and `StatefulWidget` share the
        // same method name `render`.
        StatefulWidget::render(list, area, buf, &mut self.todo_list.state);
    }

    fn render_selected_item(&self, area: Rect, buf: &mut Buffer) {
        // We get the info depending on the item's state.
        let info = if let Some(i) = self.todo_list.state.selected() {
            match self.todo_list.items[i].status {
                Status::Completed => format!("✓ DONE: {}", self.todo_list.items[i].info),
                Status::InProgress => format!("✍ IN PROGRESS : {}", self.todo_list.items[i].info),
                Status::Todo => format!("☐ TODO: {}", self.todo_list.items[i].info),
            }
        } else {
            "Nothing selected...".to_string()
        };

        // We show the list item's info under the list in this paragraph
        let block = Block::new()
            .title(Line::raw("TODO Info").centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(TODO_HEADER_STYLE)
            .bg(NORMAL_ROW_BG)
            .padding(Padding::horizontal(1));

        // We can now render the item info
        Paragraph::new(info)
            .block(block)
            .fg(TEXT_FG_COLOR)
            .wrap(Wrap { trim: false })
            .render(area, buf);
    }

    fn render_history(&mut self, area: Rect, buf: &mut Buffer) {
        // We show the list item's info under the list in this paragraph
        let block = Block::new()
            .title(Line::raw("TODO History").centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(TODO_HEADER_STYLE)
            .bg(NORMAL_ROW_BG)
            .padding(Padding::horizontal(1));

        let items: Vec<ListItem> = self
            .history_list
            .items
            .iter()
            .enumerate()
            .map(|(i, todo_item)| {
                let color = alternate_colors(i);
                ListItem::from(todo_item).bg(color)
            })
            .collect();

        // Create a List from all list items and highlight the currently selected one
        let list = List::new(items)
            .block(block)
            .highlight_style(SELECTED_STYLE)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        // We need to disambiguate this trait method as both `Widget` and `StatefulWidget` share the
        // same method name `render`.
        StatefulWidget::render(list, area, buf, &mut self.history_list.state);
    }
}

const fn alternate_colors(i: usize) -> Color {
    if i % 2 == 0 {
        NORMAL_ROW_BG
    } else {
        ALT_ROW_BG_COLOR
    }
}

impl From<&TodoItem> for ListItem<'_> {
    fn from(value: &TodoItem) -> Self {
        let line = match value.status {
            Status::Todo => Line::styled(format!(" ☐ {}", value.todo), TEXT_FG_COLOR),
            Status::Completed => {
                Line::styled(format!(" ✓ {}", value.todo), COMPLETED_TEXT_FG_COLOR)
            }
            Status::InProgress => {
                Line::styled(format!(" ✍ {}", value.todo), IN_PROGRESS_TEXT_FG_COLOR)
            }
        };
        ListItem::new(line)
    }
}
