use crate::processing::run_all;
use ratatui::crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    layout::{Constraint, Layout},
    style::{Color, Style},
    text::Span,
    widgets::{Block, List, ListItem, ListState, Paragraph},
    Frame,
};
use std::io;

/// Selectable List of Objects
pub struct StatefulList<T> {
    items: Vec<T>,
    state: ListState,
    selected_items: Vec<usize>,
}

impl<T> StatefulList<T> {
    /// Create a new StatefulList from a Vector
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            items,
            state: ListState::default(),
            selected_items: Vec::new(),
        }
    }

    /// Initialize the state by selecting the first object of the items Vector
    pub fn initialize(&mut self) {
        self.state.select(Some(0));
    }

    /// Select the next item
    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    /// Select the previous item
    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    /// Set the list as focused or unfocused
    fn toggle_focus(&mut self) {
        if self.state.selected().is_some() {
            self.state.select(None);
        } else {
            self.state.select(Some(0));
        }
    }

    /// Add or remove the current item to the selected items
    fn toggle_selection(&mut self) {
        if let Some(i) = self.state.selected() {
            if self.selected_items.contains(&i) {
                // remove the previously selected item from the list
                self.selected_items.retain(|&x| x != i);
            } else {
                // add the selected item to the list
                self.selected_items.push(i);
            }
        }
    }

    /// Remove all the selected items
    fn clear_selections(&mut self) {
        self.selected_items.clear();
    }

    /// Add all items to the selected items
    fn select_all(&mut self) {
        self.clear_selections();
        for (index, _value) in self.items.iter().enumerate() {
            self.selected_items.push(index);
        }
    }
}

/// Enum of Lists types, used to keep only one list in focus
enum ViewLists {
    Packages,
    Distros,
    Confirm,
}

/// Ui
pub struct Ui {
    pub packages_list: StatefulList<String>,
    pub distros_list: StatefulList<String>,
}

impl Ui {
    /// Initialize the Ui
    pub fn initialize(&mut self) -> io::Result<()> {
        // terminal init
        enable_raw_mode()?;
        let _ = io::stdout().execute(EnterAlternateScreen);
        // initialize the first selected item
        self.distros_list.initialize();

        Ok(())
    }

    /// Close the Ui
    pub fn exit(&mut self) -> io::Result<()> {
        disable_raw_mode()?;
        io::stdout().execute(LeaveAlternateScreen)?;
        Ok(())
    }

    //--------------------------------------------------SELECTION STATE---------------------------

    /// Handle user's commands in the selection state
    pub fn handle_selection_events(
        &mut self,
        confirm_message: String,
    ) -> io::Result<(bool, String)> {
        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                let active_list: ViewLists;

                active_list = self.find_active();

                match key.code {
                    // quit
                    KeyCode::Char('q') => return Ok((true, "".to_string())),
                    // move down in the list
                    KeyCode::Down | KeyCode::Char('j') => {
                        match active_list {
                            ViewLists::Packages => self.packages_list.next(),
                            ViewLists::Distros => self.distros_list.next(),
                            _ => {}
                        }
                        return Ok((false, confirm_message));
                    }
                    // move up in the list
                    KeyCode::Up | KeyCode::Char('k') => {
                        match active_list {
                            ViewLists::Packages => self.packages_list.previous(),
                            ViewLists::Distros => self.distros_list.previous(),
                            _ => {}
                        }
                        return Ok((false, confirm_message));
                    }
                    // select the current item
                    KeyCode::Char(' ') => {
                        match active_list {
                            ViewLists::Packages => self.packages_list.toggle_selection(),
                            ViewLists::Distros => {
                                // only one selection at a time
                                self.distros_list.clear_selections();
                                self.distros_list.toggle_selection()
                            }
                            _ => {}
                        }
                        return Ok((false, confirm_message));
                    }
                    // move horizontally to the left
                    KeyCode::Left | KeyCode::Char('h') => {
                        match active_list {
                            ViewLists::Distros => {
                                self.distros_list.toggle_focus();
                                self.packages_list.toggle_focus();
                            }
                            _ => {}
                        }
                        return Ok((false, confirm_message));
                    }
                    // move horizontally to the right
                    KeyCode::Right | KeyCode::Char('l') => {
                        match active_list {
                            ViewLists::Packages => {
                                self.distros_list.toggle_focus();
                                self.packages_list.toggle_focus();
                            }
                            _ => {}
                        }
                        return Ok((false, confirm_message));
                    }
                    // switch list in focus
                    KeyCode::Tab => {
                        self.distros_list.toggle_focus();
                        self.packages_list.toggle_focus();
                        return Ok((false, confirm_message));
                    }
                    // shift + a selects all packages
                    KeyCode::Char('a') => {
                        self.packages_list.select_all();
                        return Ok((false, confirm_message));
                    }
                    // remove all selections
                    KeyCode::Char('d') => {
                        self.packages_list.clear_selections();
                        return Ok((false, confirm_message));
                    }
                    // confirm the choices
                    KeyCode::Enter => {
                        let message: String;
                        if self.distros_list.selected_items.is_empty() {
                            message = "You must select your current distro".to_string();
                        } else {
                            message = "Do you want to proceed? y/n".to_string();
                            // focus the confirm section
                            match active_list {
                                ViewLists::Packages => self.packages_list.toggle_focus(),
                                ViewLists::Distros => self.distros_list.toggle_focus(),
                                _ => {}
                            }
                        }

                        return Ok((false, message));
                    }
                    // execute the commands
                    KeyCode::Char('y') => {
                        match active_list {
                            // TODO: get only the selected scripts to run
                            ViewLists::Confirm => {
                                let _ = run_all(self.packages_list.items.clone());
                            }
                            _ => {}
                        }
                        return Ok((false, confirm_message));
                    }
                    // focus the packages list
                    KeyCode::Char('n') => {
                        match active_list {
                            ViewLists::Confirm => self.packages_list.state.select(Some(0)),
                            _ => {}
                        }
                        // reset the message in the confirm bar
                        return Ok((false, "".to_string()));
                    }
                    // default case
                    _ => return Ok((false, confirm_message)),
                }
            }
        }
        return Ok((false, confirm_message));
    }

    /// Draw the Ui in the selection state
    pub fn selection_ui(&mut self, frame: &mut Frame, confirm_message: String) {
        const GREETINGS_TEXT: &str = "
        __    _             __  _ __    
       / /   (_)___  __  __/ /_(_) /____
      / /   / / __ \\/ / / / __/ / / ___/
     / /___/ / / / / /_/ / /_/ / (__  ) 
    /_____/_/_/ /_/\\__,_/\\__/_/_/____/  

\n
 Use the arrow keys or vim motion keys (h,j,k,l) to navigate the lists
 Use <Tab> to switch between lists
 Use <Space> to select the highligthted item
 Use <Enter> to confirm your choices
 Use <a> to select all packages
 Use <d> to deselect all packages
 Use <q> to quit";

        let [title_area, main_area, status_area] = Layout::vertical([
            Constraint::Length(18),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .areas(frame.area());
        let [left_area, right_area] =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                .areas(main_area);

        frame.render_widget(
            Paragraph::new(GREETINGS_TEXT).block(Block::bordered()),
            title_area,
        );
        frame.render_widget(
            Paragraph::new(confirm_message).block(Block::bordered().title("Confirm")),
            status_area,
        );

        const HIGHLIGHTED_STYLE: Style = Style::new().fg(Color::LightGreen).bg(Color::DarkGray);

        let packages: Vec<ListItem> = Self::to_list_items(
            &self.packages_list.items,
            self.packages_list.selected_items.clone(),
        );

        let packages_widget = List::new(packages)
            .block(Block::bordered().title("Select what to install and setup"))
            .highlight_style(HIGHLIGHTED_STYLE);

        frame.render_stateful_widget(packages_widget, left_area, &mut self.packages_list.state);

        let distros: Vec<ListItem> = Self::to_list_items(
            &self.distros_list.items,
            self.distros_list.selected_items.clone(),
        );

        let distros_widget = List::new(distros)
            .block(Block::bordered().title("Choose your current distro"))
            .highlight_style(HIGHLIGHTED_STYLE);

        frame.render_stateful_widget(distros_widget, right_area, &mut self.distros_list.state);
    }

    //---------------------------------------------UTILITY FUNCTIONS------------------------------

    /// Find the list that is currently in focus
    fn find_active(&mut self) -> ViewLists {
        if self.packages_list.state.selected().is_some() {
            ViewLists::Packages
        } else if self.distros_list.state.selected().is_some() {
            ViewLists::Distros
        } else {
            // default case
            ViewLists::Confirm
        }
    }

    /// Turn an array of strings into a Vector of ListItems
    fn to_list_items(items: &[String], selected_items: Vec<usize>) -> Vec<ListItem> {
        items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let checkbox = if selected_items.contains(&i) {
                    "[x] "
                } else {
                    "[ ] "
                };
                let content = format!("{}{}", checkbox, item);
                let style = if selected_items.contains(&i) {
                    Style::default().fg(Color::LightGreen)
                } else {
                    Style::default()
                };
                ListItem::new(Span::from(content)).style(style)
            })
            .collect()
    }
}
