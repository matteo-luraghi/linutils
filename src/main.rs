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

    fn toggle_selection(&mut self) {
        if let Some(i) = self.state.selected() {
            if self.selected_items.contains(&i) {
                self.selected_items.retain(|&x| x != i);
            } else {
                self.selected_items.push(i);
            }
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
    enable_raw_mode()?;
    let _ = stdout().execute(EnterAlternateScreen);
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let items = vec![
        "alacritty".to_string(),
        "neovim".to_string(),
        "hyprland".to_string(),
        "zsh".to_string(),
        "docker".to_string(),
        "webeep-sync".to_string(),
        "discord".to_string(),
        "fzf".to_string(),
    ];
    let mut list = StatefulList::with_items(items);

    let distros = vec!["fedora".to_string(), "ubuntu".to_string()];
    let mut distros_list = StatefulList::with_items(distros);

    let mut should_quit = false;
    while !should_quit {
        terminal.draw(|f| ui(f, &mut list, &mut distros_list))?;
        should_quit = handle_events(&mut list, &mut distros_list)?;
    }
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    return Ok(());
}

fn handle_events(
    list: &mut StatefulList<String>,
    distros_list: &mut StatefulList<String>,
) -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            let active_list: &mut StatefulList<String>;
            let inactive_list: &mut StatefulList<String>;

            if list.state.selected() != None {
                active_list = list;
                inactive_list = distros_list;
            } else {
                active_list = distros_list;
                inactive_list = list;
            };

            match key.code {
                // quit
                KeyCode::Char('q') => return Ok(true),
                KeyCode::Down => {
                    active_list.next();
                    return Ok(false);
                }
                KeyCode::Up => {
                    active_list.previous();
                    return Ok(false);
                }
                KeyCode::Enter => {
                    active_list.toggle_selection();
                    return Ok(false);
                }
                // switch list in focus
                KeyCode::Tab => {
                    active_list.toggle_focus();
                    inactive_list.toggle_focus();
                    return Ok(false);
                }
                _ => return Ok(false), // default case
            }
        }
    }
    return Ok(false);
}

fn ui(frame: &mut Frame, list: &mut StatefulList<String>, distros_list: &mut StatefulList<String>) {
    let [title_area, main_area, status_area] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(0),
        Constraint::Length(3),
    ])
    .areas(frame.area());
    let [left_area, right_area] =
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
            .areas(main_area);

    frame.render_widget(
        Paragraph::new("LINUTILS").block(Block::bordered()),
        title_area,
    );
    frame.render_widget(Block::bordered().title("Status Bar"), status_area);

    let highlighted_style: Style = Style::default().fg(Color::LightGreen).bg(Color::DarkGray);

    let commands: Vec<ListItem> = to_list_items(&list.items, list.selected_items.clone());

    let commands_widget = List::new(commands)
        .block(Block::bordered().title("Select the packages to install and configure automatically"))
        .highlight_style(highlighted_style);

    frame.render_stateful_widget(commands_widget, left_area, &mut list.state);

    let distros: Vec<ListItem> =
        to_list_items(&distros_list.items, distros_list.selected_items.clone());

    let distros_widget = List::new(distros)
        .block(Block::bordered().title("Choose your current distro"))
        .highlight_style(highlighted_style);

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
