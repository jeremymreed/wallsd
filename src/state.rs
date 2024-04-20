use std::collections::HashMap;
use std::sync::mpsc::{self, Sender, Receiver};
use crate::output::Output;
use crate::config::Config;
use crate::mode;
use crate::command::InternalCommand;

// The program's current state.
pub struct State {
    pub config: Config,
    pub outputs: HashMap<String, Output>,
}

impl State {
    pub fn new() -> State {
        State {
            config: Config::load_config(),
            outputs: HashMap::new(),
        }
    }

    pub fn set_mode(&mut self, name: &String, mode: mode::Mode) {
        let output = self.outputs.get_mut(name).unwrap();

        output.mode = mode;
    }
}
