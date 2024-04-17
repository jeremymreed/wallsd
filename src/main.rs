use std::collections::HashMap;
use std::sync::mpsc;
use std::{thread, time::Duration};
use chrono::Local;
use futures::executor::block_on;
use crate::output::Output;

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

    let config = config::Config::load_config();

    let (tx, rx) = mpsc::channel::<command::InternalCommand>();

    let outputs = swaymsg::get_outputs();
    tracing::debug!("Found outputs: {:#?}", outputs);
    tracing::info!("Loaded outputs");

    let mut outputs_map: HashMap<String, Output> = HashMap::new();

    for output in &outputs {
        outputs_map.insert(output.name.clone(), output.clone());
    }

    let mut collection: collection::Collection = collection::Collection {
        collection: Vec::new(),
    };

    collection.scan_collection(&config.default_wallpaper_collection);

    for output in outputs_map.values_mut() {
        output.images = collection.collection.clone();
    }

    thread::spawn(|| {
        tracing::info!("Starting dbus server");
        let _ = block_on(dbus_server::run_server(tx));
    });

    let sleep_duration = Duration::from_secs(1);

    tracing::debug!("oncalendar_string: {:?}", config.oncalendar_string);

    outputs_map.get_mut("HDMI-A-1").unwrap().oncalendar_string = String::from("*-*-* *:0/2");
    outputs_map.get_mut("eDP-1").unwrap().oncalendar_string = String::from("*-*-* *:0/1");

    for output in outputs_map.values_mut() {
        //output.oncalendar_string = config.oncalendar_string.clone();
        output.target_time = systemd_analyze::get_next_event(&output.oncalendar_string);
    }

    tracing::info!("Starting main loop");
    loop {
        let ghetto_profiler = profiler::Profiler::start();

        let current_time = Local::now();

        tracing::trace!("Checking for dbus events!");
        match rx.try_recv() {
            Ok(message) => {
                match message {
                    command::InternalCommand::SetOutputModeCommand(command) => {
                        tracing::debug!("Recieved SetOutputModeCommand: {:#?}", command);
                    },
                    _ => {
                        tracing::debug!("Recieved unknown command!");
                    }
                }
            },
            Err(_) => tracing::debug!("No message"),
        };

        for output in outputs_map.values_mut() {
            // Check to see if the timer should be fired.
            if on_calendar::is_time_after_target(output.target_time, current_time) {
                tracing::debug!("******  TIMER FIRED *******");
                swww::set_wallpaper(output);
                output.target_time = systemd_analyze::get_next_event(&output.oncalendar_string);
            }
        }

        ghetto_profiler.stop();

        thread::sleep(sleep_duration);
    }
}
