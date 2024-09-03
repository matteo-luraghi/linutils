mod config;
mod processing;
mod tui;

use crate::config::load_config;
use crate::tui::{StatefulList, Ui};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

// MAIN
fn main() -> io::Result<()> {
    let config = load_config("./src/config.toml");
    // lists init
    let packages = config.packages;
    let distros = config
        .distros
        .into_iter()
        .map(|distro| distro.name)
        .collect();

    let packages_list = StatefulList::with_items(packages);
    let distros_list = StatefulList::with_items(distros);

    let mut ui = Ui {
        packages_list,
        distros_list,
        process_items_list: vec![],
    };

    // initialize the ui
    match ui.initialize() {
        Ok(()) => {}
        Err(error) => return Err(error),
    }

    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
    let mut user_interrupt = false;
    let mut should_quit = false;
    let mut confirm_message = "".to_string();

    //------------------SELECTION STATE--------------
    // screen drawing
    while !user_interrupt && !should_quit {
        // draw the terminal
        terminal.draw(|f| should_quit = ui.selection_ui(f, confirm_message.clone()))?;

        // read new values
        (user_interrupt, confirm_message) = ui.handle_selection_events(confirm_message.clone())?;
    }

    //-----------------PROCESSING STATE--------------
    // reset while condition
    should_quit = false;

    while !user_interrupt && !should_quit {
        // draw the terminal
        terminal.draw(|f| should_quit = ui.processing_ui(f))?;

        // read the new value
        user_interrupt = ui.handle_processing_events()?;
    }

    //-------------------ENDING STATE----------------

    // exit only when the user wants to
    while !user_interrupt {
        // keep drawing the statuses of installed packages
        terminal.draw(|f| {
            should_quit = ui.processing_ui(f);
        })?;

        // read the new value
        user_interrupt = ui.handle_ending_events()?;
    }

    //-----------------------EXIT---------------------

    // close the ui
    match ui.exit() {
        Ok(()) => {}
        Err(error) => return Err(error),
    }

    return Ok(());
}
