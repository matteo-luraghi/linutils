mod processing;
mod tui;

use crate::tui::{StatefulList, Ui};
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{Clear, ClearType};
use ratatui::{backend::CrosstermBackend, Terminal};
use serde::Deserialize;
use std::fs;
use std::io::{self};

/// Config arguments
#[derive(Deserialize, Debug)]
struct Config {
    packages: Vec<String>,
    distros: Vec<String>,
}

/// Load the config.toml file into a Config object
fn load_config(file_path: &str) -> Config {
    let config_content = fs::read_to_string(file_path).expect("Failed to read config file");

    toml::from_str(&config_content).expect("Failed to parse config file")
}

// MAIN
fn main() -> io::Result<()> {
    let config = load_config("./src/config.toml");
    // lists init
    let packages = config.packages;
    let distros = config.distros;

    let packages_list = StatefulList::with_items(packages);
    let distros_list = StatefulList::with_items(distros);

    let mut ui = Ui {
        packages_list,
        distros_list,
        packages_items_list: vec![],
    };

    // initialize the ui
    match ui.initialize() {
        Ok(()) => {},
        Err(error) => return Err(error),
    }

    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
    let mut should_quit = false;
    let mut confirm_message = "".to_string();

    //------------------SELECTION STATE--------------
    // screen drawing
    while !should_quit {
        // draw the terminal
        terminal.draw(|f| ui.selection_ui(f, confirm_message.clone()))?;

        // read new values
        (should_quit, confirm_message) = ui.handle_selection_events(confirm_message.clone())?;
    }

    // set the selected packages in the ui
    let selected_packages = ui.packages_list.get_selected_items();
    ui.set_packages_items_list(&selected_packages);

    //-----------------PROCESSING STATE--------------
    // clear the screen
    execute!(terminal.backend_mut(), Clear(ClearType::All))?;
    should_quit = false;

    while !should_quit {
        // draw the terminal
        terminal.draw(|f| ui.processing_ui(f))?;

        // read the new value
        should_quit = ui.handle_processing_events()?;
    }

    // close the ui
    match ui.exit() {
        Ok(()) => {},
        Err(error) => {return Err(error)},
    }

    return Ok(());
}
