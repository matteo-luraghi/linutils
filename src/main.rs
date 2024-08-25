use std::env::args;
use std::io::{self, stdout};
use std::process::Command;

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
        if self.state.selected() != None {
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

fn exec_command() {
    let distro = args().nth(1).expect("Error: no distro selected");
    println!("{}", distro);

    let script_path = format!("./{}/apps/discord.sh", distro);

    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("yes | sudo {}", script_path)) // repeatedly says yes to every prompt
        .output()
        .expect("failed to execute process");
    println!("{:?}", output)
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
    while !should_quit {
        terminal.draw(|f| ui(f, &mut packages_list, &mut distros_list))?;
        should_quit = handle_events(&mut packages_list, &mut distros_list)?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    return Ok(());
}

fn handle_events(
    packages_list: &mut StatefulList<String>,
    distros_list: &mut StatefulList<String>,
) -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            enum ViewLists {
                Packages,
                Distros,
                // Confirm, // to be implemented for the popup confirmation menu
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
                    ViewLists::Distros
                }
            }

            let active_list: ViewLists;

            active_list = find_active(packages_list, distros_list);

            match key.code {
                // quit
                KeyCode::Char('q') => return Ok(true),
                // confirm the choices
                KeyCode::Enter => {
                    // TODO
                    // - check for errors in selection of distro
                    // - open a popup "Confirm y/n"
                    // - if confirmed exec all the needed bash scripts
                    return Ok(false);
                }
                // move down in the list
                KeyCode::Down | KeyCode::Char('j') => {
                    match active_list {
                        ViewLists::Packages => packages_list.next(),
                        ViewLists::Distros => distros_list.next(),
                    }
                    return Ok(false);
                }
                // move up in the list
                KeyCode::Up | KeyCode::Char('k') => {
                    match active_list {
                        ViewLists::Packages => packages_list.previous(),
                        ViewLists::Distros => distros_list.previous(),
                    }
                    return Ok(false);
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
                    }
                    return Ok(false);
                }
                // move horizontally to the right
                KeyCode::Char('h') => match active_list {
                    ViewLists::Distros => {
                        distros_list.toggle_focus();
                        packages_list.toggle_focus();
                    }
                    _ => {}
                },
                // move horizontally to the left
                KeyCode::Char('l') => match active_list {
                    ViewLists::Packages => {
                        distros_list.toggle_focus();
                        packages_list.toggle_focus();
                    }
                    _ => {}
                },
                // switch list in focus
                KeyCode::Tab => {
                    distros_list.toggle_focus();
                    packages_list.toggle_focus();
                    return Ok(false);
                }
                // shift + a selects all packages
                KeyCode::Char('a') => packages_list.select_all(),

                // remove all selections
                KeyCode::Char('d') => packages_list.clear_selections(),
                _ => return Ok(false), // default case
            }
        }
    }
    return Ok(false);
}

fn ui(frame: &mut Frame, list: &mut StatefulList<String>, distros_list: &mut StatefulList<String>) {
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
    frame.render_widget(Block::bordered().title("Confirm"), status_area);

    const HIGHLIGHTED_STYLE: Style = Style::new().fg(Color::LightGreen).bg(Color::DarkGray);

    let commands: Vec<ListItem> = to_list_items(&list.items, list.selected_items.clone());

    let commands_widget = List::new(commands)
        .block(
            Block::bordered().title("Select the packages to install and configure automatically"),
        )
        .highlight_style(HIGHLIGHTED_STYLE);

    frame.render_stateful_widget(commands_widget, left_area, &mut list.state);

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
