use std::collections::HashMap;
use std::sync::mpsc;
use std::{thread, time::Duration};
use chrono::Local;
use futures::executor::block_on;
use crate::state::State;
use crate::output::Output;
use crate::mode::Mode;

mod state;
mod status;
mod image_verification;
mod profiler;
mod logging;
mod config;
mod command;
mod mode;
mod output;
mod resolution;
mod dbus_server;
mod systemd_analyze;
mod on_calendar;
mod swaymsg;
mod swww;
mod collection;

#[async_std::main]
async fn main() {

    logging::init();

    let mut state = State::new();

    swaymsg::get_outputs(&mut state);
    tracing::debug!("Found outputs: {:#?}", state.outputs);
    tracing::info!("Loaded outputs");

    let mut collection: collection::Collection = collection::Collection {
        collection: Vec::new(),
    };

    collection.scan_collection(&state.config.default_wallpaper_collection);

    for output in state.outputs.values_mut() {
        output.images = collection.collection.clone();
    }

    thread::spawn(|| {
        tracing::info!("Starting dbus server");
        let _ = block_on(dbus_server::run_server(state.tx));
    });

    let sleep_duration = Duration::from_secs(1);

    tracing::debug!("oncalendar_string: {:?}", state.config.oncalendar_string);

    state.outputs.get_mut("HDMI-A-1").unwrap().oncalendar_string = String::from("*-*-* *:0/2");
    state.outputs.get_mut("eDP-1").unwrap().oncalendar_string = String::from("*-*-* *:0/1");

    for output in state.outputs.values_mut() {
        //output.oncalendar_string = config.oncalendar_string.clone();
        output.target_time = systemd_analyze::get_next_event(&output.oncalendar_string);
    }

    tracing::info!("Starting main loop");
    loop {
        let ghetto_profiler = profiler::Profiler::start();

        let current_time = Local::now();

        tracing::trace!("Checking for dbus events!");
        match state.rx.try_recv() {
            Ok(message) => {
                match message {
                    command::InternalCommand::SetOutputModeCommand(command) => {
                        tracing::debug!("Recieved SetOutputModeCommand: {:#?}", command);
                        let output = state.outputs.get_mut(&command.output).unwrap();
                        output.mode = command.mode;
                    },
                    _ => {
                        tracing::debug!("Recieved unknown command!");
                    }
                }
            },
            Err(_) => tracing::debug!("No message"),
        };

        for output in state.outputs.values_mut() {
            tracing::debug!("Checking output: {:#?}", output.name);
            // Check to see if the timer should be fired.
            if output.mode == Mode::Slideshow && on_calendar::is_time_after_target(output.target_time, current_time) {
                tracing::debug!("******  TIMER FIRED *******");
                swww::set_wallpaper(output);
                output.target_time = systemd_analyze::get_next_event(&output.oncalendar_string);
            } else if output.mode == Mode::Oneshot {
                tracing::debug!("******  ONESHOT MODE *******");
            }
            tracing::debug!("Done checking output");
        }

        ghetto_profiler.stop();

        thread::sleep(sleep_duration);
    }
}
