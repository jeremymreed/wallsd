use std::collections::HashMap;
use std::sync::mpsc::{self, Sender, Receiver};
use crate::output::Output;
use crate::config::Config;
use crate::command::InternalCommand;

// The program's current state.
pub struct State {
    pub tx: Sender<InternalCommand>,
    pub rx: Receiver<InternalCommand>,
    pub config: Config,
    pub outputs: HashMap<String, Output>,
}

impl State {
    pub fn new() -> State {
        State {
            tx: mpsc::channel::<InternalCommand>().0,
            rx: mpsc::channel::<InternalCommand>().1,
            config: Config::load_config(),
            outputs: HashMap::new(),
        }
    }
}
