use crate::logging;
use crate::command;
use crate::on_calendar;
use crate::mode::Mode;
use crate::state::State;
use crate::swaymsg;
use crate::swww;
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

        let mut collection: collection::Collection = collection::Collection::new();

        collection.scan_collection(&self.state.config.default_wallpaper_collection);

        for output in self.state.outputs.values_mut() {
            output.images = collection.collection.clone();
        }

        collection.collection.clear();

        tracing::debug!("oncalendar_string: {:?}", self.state.config.oncalendar_string);

        // Tempory hack.
        self.state.outputs.get_mut("HDMI-A-1").unwrap().oncalendar_string = String::from("*-*-* *:0/2");
        self.state.outputs.get_mut("eDP-1").unwrap().oncalendar_string = String::from("*-*-* *:0/1");

        for output in self.state.outputs.values_mut() {
            //output.oncalendar_string = config.oncalendar_string.clone();
            output.target_time = match systemd_analyze::get_next_event(&output.oncalendar_string) {
                Ok(time) => time,
                Err(_) => {
                    tracing::error!("Failed to get next event for output: {:#?}", output.name);
                    panic!("Failed to get next event for output: {:#?}", output.name);
                },
            };
        }
    }

    pub fn poll_dbus_messages(&mut self, message: command::InternalCommand) -> Result<command::InternalCommand, command::InternalCommand> {
        match message {
            command::InternalCommand::SetOutputModeCommand(command) => {
                tracing::debug!("Recieved SetOutputModeCommand: {:#?}", command);
                self.state.set_mode(&command.output, command.mode)
            },
            command::InternalCommand::SetOutputOncalendarCommand(command) => {
                tracing::debug!("Recieved SetOutputOncalendarCommand: {:#?}", command);

                self.state.set_oncalendar(&command.output, command.oncalendar)
            }
            command::InternalCommand::SetOutputImagesCommand(command) => {
                tracing::debug!("Recieved SetOutputImagesCommand: {:#?}", command);
                self.state.set_images(&command.output, command.images)
            },
            command::InternalCommand::GetOutputsSettingsCommand => {
                tracing::debug!("Recieved GetOutputsSettingsCommand");
                self.state.get_outputs_settings()
            },
            _ => {
                tracing::debug!("Recieved unknown command!");
                let response = command::GeneralResponse {
                    status: crate::status::Status::Failure,
                    error: "Unknown command".to_string(),
                };
                Err(command::InternalCommand::GeneralResponse(response))
            }
        }
    }

    pub fn check_outputs(&mut self, current_time: chrono::DateTime<chrono::Local>) {
        for output in self.state.outputs.values_mut() {
            tracing::debug!("Checking output: {:#?}", output.name);
            // Check to see if the timer should be fired.
            match output.mode {
                Mode::Slideshow => {
                    tracing::debug!("******  SLIDESHOW MODE *******");
                    tracing::debug!("images.len(): {:#?}", output.images.len());
                    if on_calendar::is_time_after_target(output.target_time, current_time) {
                        tracing::debug!("******  TIMER FIRED *******");
                        //self.state.set_wallpaper(&output.name);
                        output.current_wallpaper = swww::set_wallpaper(output);
                        output.target_time = match systemd_analyze::get_next_event(&output.oncalendar_string) {
                            Ok(time) => time,
                            Err(_) => {
                                tracing::error!("Failed to get next event for output: {:#?}", output.name);
                                panic!("Failed to get next event for output: {:#?}", output.name);
                            },
                        };
                    }
                },
                Mode::Oneshot => {
                    tracing::debug!("******  ONESHOT MODE *******");
                },
            }

            tracing::debug!("Done checking output");
        }
    }
}

