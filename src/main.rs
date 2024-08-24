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
        "Item 1".to_string(),
        "Item 2".to_string(),
        "Item 3".to_string(),
        "Item 4".to_string(),
        "Item 5".to_string(),
        "Item 6".to_string(),
        "Item 7".to_string(),
        "Item 8".to_string(),
        "Item 9".to_string(),
    ];
    let mut list = StatefulList::with_items(items);

    let mut should_quit = false;
    while !should_quit {
        terminal.draw(|f| ui(f, &mut list))?;
        should_quit = handle_events(&mut list)?;
    }
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    return Ok(());
}

fn handle_events(list: &mut StatefulList<String>) -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(true),
                KeyCode::Down => {
                    list.next();
                    return Ok(false);
                }
                KeyCode::Up => {
                    list.previous();
                    return Ok(false);
                }
                KeyCode::Enter => {
                    list.toggle_selection();
                    return Ok(false);
                }
                _ => return Ok(false), // default case
            }
        }
    }
    return Ok(false);
}

fn ui(frame: &mut Frame, list: &mut StatefulList<String>) {
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

    let items: Vec<ListItem> = list
        .items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let checkbox = if list.selected_items.contains(&i) {
                "[x]"
            } else {
                "[ ]"
            };
            let content = format!("{}{}", checkbox, item);
            let style = if list.selected_items.contains(&i) {
                Style::default().fg(Color::LightGreen)
            } else {
                Style::default()
            };
            ListItem::new(Span::from(content)).style(style)
        })
        .collect();

    let list_widget = List::new(items)
        .block(Block::bordered().title("Left"))
        .highlight_style(Style::default().fg(Color::LightGreen).bg(Color::Black));

    frame.render_stateful_widget(list_widget, left_area, &mut list.state);

    frame.render_widget(Block::bordered().title("Right"), right_area);
}
