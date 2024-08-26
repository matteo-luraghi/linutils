use std::io::{self, stdout};
use std::process::{Command, Stdio};

use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, Event, KeyCode},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    layout::{Constraint, Layout},
    style::{Color, Style},
    text::Span,
    widgets::{Block, List, ListItem, ListState, Paragraph},
    Frame, Terminal,
};

fn exec_script(path: &str) {
    let script_name_optional = path.split("/").last();

    let script_name = match script_name_optional {
        Some(name) => name,
        None => {
            println!("Failed to extract script name");
            return;
        }
    };

    let mut script = Command::new("sh");
    let output = script
        .arg("-c")
        .arg(format!("./{}", path))
        .stdout(Stdio::piped())
        .output()
        .expect(&format!("Error executing script {}", script_name));

    if script.status().expect("Failed to execute script").success() {
        println!("Command executed correctly");
    } else {
        println!("Error executing script {}", script_name);
        println!("{:?}", output);
    }
}

struct StatefulList<T> {
    items: Vec<T>,
    state: ListState,
    selected_items: Vec<usize>,
}

impl<T> StatefulList<T> {
    fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            items,
            state: ListState::default(),
            selected_items: Vec::new(),
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
            None => 0,
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
            None => 0,
        };
        self.state.select(Some(i));
    }

    // set the list as focused or unfocused
    fn toggle_focus(&mut self) {
        if self.state.selected().is_some() {
            self.state.select(None);
        } else {
            self.state.select(Some(0));
        }
    }

    // add or remove the current item to the selected items
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

    // remove all the selected items
    fn clear_selections(&mut self) {
        self.selected_items.clear();
    }

    // add all items to the selected items
    fn select_all(&mut self) {
        self.clear_selections();
        for (index, _value) in self.items.iter().enumerate() {
            self.selected_items.push(index);
        }
    }
}

// MAIN
fn main() -> io::Result<()> {
    // init
    enable_raw_mode()?;
    let _ = stdout().execute(EnterAlternateScreen);
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    // lists init
    let packages = vec![
        "alacritty".to_string(),
        "neovim".to_string(),
        "hyprland".to_string(),
        "zsh".to_string(),
        "docker".to_string(),
        "webeep-sync".to_string(),
        "discord".to_string(),
        "fzf".to_string(),
    ];
    let mut packages_list = StatefulList::with_items(packages);

    let distros = vec!["fedora".to_string(), "ubuntu".to_string()];
    let mut distros_list = StatefulList::with_items(distros);
    // initialize the first selected item
    distros_list.state.select(Some(0));

    // screen drawing
    let mut should_quit = false;
    let mut confirm_message = "".to_string();

    while !should_quit {
        // draw the terminal
        terminal.draw(|f| {
            ui(
                f,
                &mut packages_list,
                &mut distros_list,
                confirm_message.clone(),
            )
        })?;

        // read new values
        (should_quit, confirm_message) = handle_events(
            &mut packages_list,
            &mut distros_list,
            confirm_message.clone(),
        )?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    return Ok(());
}

fn handle_events(
    packages_list: &mut StatefulList<String>,
    distros_list: &mut StatefulList<String>,
    confirm_message: String,
) -> io::Result<(bool, String)> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            enum ViewLists {
                Packages,
                Distros,
                Confirm,
            }

            // find the list that is currently in focus
            fn find_active(
                packages_list: &mut StatefulList<String>,
                distros_list: &mut StatefulList<String>,
            ) -> ViewLists {
                if packages_list.state.selected().is_some() {
                    ViewLists::Packages
                } else if distros_list.state.selected().is_some() {
                    ViewLists::Distros
                } else {
                    // default case
                    ViewLists::Confirm
                }
            }

            let active_list: ViewLists;

            active_list = find_active(packages_list, distros_list);

            match key.code {
                // quit
                KeyCode::Char('q') => return Ok((true, "".to_string())),
                // move down in the list
                KeyCode::Down | KeyCode::Char('j') => {
                    match active_list {
                        ViewLists::Packages => packages_list.next(),
                        ViewLists::Distros => distros_list.next(),
                        _ => {}
                    }
                    return Ok((false, confirm_message));
                }
                // move up in the list
                KeyCode::Up | KeyCode::Char('k') => {
                    match active_list {
                        ViewLists::Packages => packages_list.previous(),
                        ViewLists::Distros => distros_list.previous(),
                        _ => {}
                    }
                    return Ok((false, confirm_message));
                }
                // select the current item
                KeyCode::Char(' ') => {
                    match active_list {
                        ViewLists::Packages => packages_list.toggle_selection(),
                        ViewLists::Distros => {
                            // only one selection at a time
                            distros_list.clear_selections();
                            distros_list.toggle_selection()
                        }
                        _ => {}
                    }
                    return Ok((false, confirm_message));
                }
                // move horizontally to the left
                KeyCode::Left | KeyCode::Char('h') => {
                    match active_list {
                        ViewLists::Distros => {
                            distros_list.toggle_focus();
                            packages_list.toggle_focus();
                        }
                        _ => {}
                    }
                    return Ok((false, confirm_message));
                }
                // move horizontally to the right
                KeyCode::Right | KeyCode::Char('l') => {
                    match active_list {
                        ViewLists::Packages => {
                            distros_list.toggle_focus();
                            packages_list.toggle_focus();
                        }
                        _ => {}
                    }
                    return Ok((false, confirm_message));
                }
                // switch list in focus
                KeyCode::Tab => {
                    distros_list.toggle_focus();
                    packages_list.toggle_focus();
                    return Ok((false, confirm_message));
                }
                // shift + a selects all packages
                KeyCode::Char('a') => {
                    packages_list.select_all();
                    return Ok((false, confirm_message));
                }
                // remove all selections
                KeyCode::Char('d') => {
                    packages_list.clear_selections();
                    return Ok((false, confirm_message));
                }
                // confirm the choices
                KeyCode::Enter => {
                    let message: String;
                    if distros_list.selected_items.is_empty() {
                        message = "You must select your current distro".to_string();
                    } else {
                        message = "Do you want to proceed? y/n".to_string();
                        // focus the confirm section
                        match active_list {
                            ViewLists::Packages => packages_list.toggle_focus(),
                            ViewLists::Distros => distros_list.toggle_focus(),
                            _ => {}
                        }
                    }

                    return Ok((false, message));
                }
                // execute the commands
                KeyCode::Char('y') => {
                    match active_list {
                        ViewLists::Confirm => exec_script("src/commands/test.sh"),
                        _ => {}
                    }
                    return Ok((false, confirm_message));
                }
                // focus the packages list
                KeyCode::Char('n') => {
                    match active_list {
                        ViewLists::Confirm => packages_list.state.select(Some(0)),
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

fn ui(
    frame: &mut Frame,
    packages_list: &mut StatefulList<String>,
    distros_list: &mut StatefulList<String>,
    confirm_message: String,
) {
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

    let packages: Vec<ListItem> =
        to_list_items(&packages_list.items, packages_list.selected_items.clone());

    let packages_widget = List::new(packages)
        .block(
            Block::bordered().title("Select the packages to install and configure automatically"),
        )
        .highlight_style(HIGHLIGHTED_STYLE);

    frame.render_stateful_widget(packages_widget, left_area, &mut packages_list.state);

    let distros: Vec<ListItem> =
        to_list_items(&distros_list.items, distros_list.selected_items.clone());

    let distros_widget = List::new(distros)
        .block(Block::bordered().title("Choose your current distro"))
        .highlight_style(HIGHLIGHTED_STYLE);

    frame.render_stateful_widget(distros_widget, right_area, &mut distros_list.state);
}

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
