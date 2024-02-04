use std::error;

use ratatui::widgets::ListState;

use crate::file_system::{collect_data, Folder};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub struct StatefulList {
    state: ListState,
    pub items: Vec<Folder>,
    last_selected: Option<usize>,
}

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// counter
    pub counter: u8,
    /// StatefulList
    pub stateful_list: StatefulList,
}

// impl Default for App {
//     fn default() -> Self {
//         Self {
//             running: true,
//             counter: 0,
//         }
//     }
// }

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        let path_r = collect_data("/home/zelak/ghq/github.com/Zelak312/dsa");
        let path = path_r.unwrap();

        Self {
            running: true,
            counter: 0,
            stateful_list: StatefulList::with_items(path.sub_folders.values().cloned().collect()),
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn increment_counter(&mut self) {
        if let Some(res) = self.counter.checked_add(1) {
            self.counter = res;
        }
    }

    pub fn decrement_counter(&mut self) {
        if let Some(res) = self.counter.checked_sub(1) {
            self.counter = res;
        }
    }
}

impl StatefulList {
    fn with_items(items: Vec<Folder>) -> StatefulList {
        StatefulList {
            state: ListState::default(),
            items: items,
            last_selected: None,
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => self.last_selected.unwrap_or(0),
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => self.last_selected.unwrap_or(0),
        };
        self.state.select(Some(i));
    }

    fn unselect(&mut self) {
        let offset = self.state.offset();
        self.last_selected = self.state.selected();
        self.state.select(None);
        *self.state.offset_mut() = offset;
    }
}
