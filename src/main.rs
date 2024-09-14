mod config;
mod processing;
mod state;
mod tui;

use crate::state::State;
use crate::tui::{StatefulList, Ui};
use ratatui::widgets::ListState;
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

fn main() -> io::Result<()> {
    // authorize the app
    let auth = processing::get_sudo_access();
    match auth {
        Ok(success) => println!("{}", success),
        Err(error) => return Err(error),
    }

    // get the configuration
    let config = config::load_config("./src/config.toml");
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
        process_list_state: ListState::default(),
    };

    // initialize the ui
    match ui.initialize() {
        Ok(()) => {}
        Err(error) => return Err(error),
    }

    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
    // true if the user decides to exit
    let mut user_interrupt = false;
    // true if state needs to be changed
    let mut next_state = false;
    // state
    let mut state = State::Selection;
    // message for the selection view
    let mut confirm_message = "".to_string();

    while !user_interrupt {
        match state {
            State::Selection => {
                terminal.draw(|f| next_state = ui.selection_ui(f, confirm_message.clone()))?;

                // read new values
                (user_interrupt, confirm_message) =
                    ui.handle_selection_events(confirm_message.clone())?;
            }
            State::Process => {
                terminal.draw(|f| next_state = ui.processing_ui(f, false))?;

                // read the new value setting ended as false
                user_interrupt = ui.handle_processing_events(false)?;
            }
            State::End => {
                // keep drawing the status of installed packages
                terminal.draw(|f| {
                    _ = ui.processing_ui(f, true);
                })?;

                // read the new value setting ended as true
                user_interrupt = ui.handle_processing_events(true)?;
            }
        }

        if next_state {
            state.next_state();
            next_state = false;
        }
    }

    let exit_message = match state {
        State::Process => "Execution interrupted, exiting now.",
        _ => "Closing Linutils...",
    };

    // close the ui and display exit message
    match ui.exit() {
        Ok(()) => {
            println!("{}", exit_message);
        }
        Err(error) => return Err(error),
    }

    Ok(())
}
