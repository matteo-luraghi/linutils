mod processing;
mod tui;

use crate::tui::{handle_events, ui, StatefulList};
use ratatui::Terminal;
use serde::Deserialize;
use std::fs;
use std::io::{self, stdout};

use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
};

#[derive(Deserialize, Debug)]
struct Config {
    packages: Vec<String>,
    distros: Vec<String>,
}

fn load_config(file_path: &str) -> Config {
    let config_content = fs::read_to_string(file_path).expect("Failed to read config file");

    toml::from_str(&config_content).expect("Failed to parse config file")
}

// MAIN
fn main() -> io::Result<()> {
    let config = load_config("./src/config.toml");
    // terminal init
    enable_raw_mode()?;
    let _ = stdout().execute(EnterAlternateScreen);
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    // lists init
    let packages = config.packages;
    let mut packages_list = StatefulList::with_items(packages);

    let distros = config.distros;
    let mut distros_list = StatefulList::with_items(distros);
    // initialize the first selected item
    distros_list.initialize();

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
