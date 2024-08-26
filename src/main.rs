mod processing;
mod tui;

use crate::tui::{handle_events, ui, StatefulList};
use ratatui::Terminal;
use std::io::{self, stdout};

use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
};

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
        "nerdfont".to_string(),
    ];
    let mut packages_list = StatefulList::with_items(packages);

    let distros = vec!["fedora".to_string(), "ubuntu".to_string()];
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
