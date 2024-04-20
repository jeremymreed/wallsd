use crate::logging;
use crate::command;
use crate::state::State;
use crate::swaymsg;
use crate::collection;
use crate::systemd_analyze;

pub struct Executor {
    pub state: State,
}

impl Executor {
    pub fn new() -> Executor {
        Executor {
            state: State::new(),
        }
    }

    pub fn init(&mut self) {
        logging::init();

        swaymsg::get_outputs(&mut self.state);
        tracing::debug!("Found outputs: {:#?}", self.state.outputs);
        tracing::info!("Loaded outputs");

        let mut collection: collection::Collection = collection::Collection {
            collection: Vec::new(),
        };

        collection.scan_collection(&self.state.config.default_wallpaper_collection);

        for output in self.state.outputs.values_mut() {
            output.images = collection.collection.clone();
        }

        tracing::debug!("oncalendar_string: {:?}", self.state.config.oncalendar_string);

        // Tempory hack.
        self.state.outputs.get_mut("HDMI-A-1").unwrap().oncalendar_string = String::from("*-*-* *:0/2");
        self.state.outputs.get_mut("eDP-1").unwrap().oncalendar_string = String::from("*-*-* *:0/1");

        for output in self.state.outputs.values_mut() {
            //output.oncalendar_string = config.oncalendar_string.clone();
            output.target_time = systemd_analyze::get_next_event(&output.oncalendar_string);
        }
    }

    pub fn poll_dbus_messages(&mut self, message: command::InternalCommand) {
        match message {
            command::InternalCommand::SetOutputModeCommand(command) => {
                tracing::debug!("Recieved SetOutputModeCommand: {:#?}", command);
                self.state.set_mode(&command.output, command.mode);
            },
            _ => {
                tracing::debug!("Recieved unknown command!");
            }
        }
    }
}

