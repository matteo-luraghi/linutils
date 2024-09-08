/// State pattern to handle the different views in the UI
pub enum State {
    Selection,
    Process,
    End,
}

impl State {
    /// update the state
    pub fn next_state(&mut self) {
        *self = match self {
            State::Selection => State::Process,
            State::Process => State::End,
            State::End => State::End,
        };
    }
}
