use std::{thread, time::Duration};
use std::sync::mpsc;
use chrono::Local;
use futures::executor::block_on;
use crate::command::InternalCommand;
use crate::executor::Executor;
use crate::mode::Mode;

mod executor;
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

    let mut executor = Executor::new();

    executor.init();

    let (tx, rx) = mpsc::channel::<InternalCommand>();

    thread::spawn(|| {
        tracing::info!("Starting dbus server");
        let _ = block_on(dbus_server::run_server(tx));
    });

    let sleep_duration = Duration::from_secs(1);

    tracing::info!("Starting main loop");
    loop {
        let ghetto_profiler = profiler::Profiler::start();

        let current_time = Local::now();

        tracing::trace!("Checking for dbus events!");
        match rx.try_recv() {
            Ok(message) => {
                executor.poll_dbus_messages(message);
            },
            Err(_) => tracing::debug!("No message"),
        };

        for output in executor.state.outputs.values_mut() {
            tracing::debug!("Checking output: {:#?}", output.name);
            // Check to see if the timer should be fired.
            match output.mode {
                Mode::Slideshow => {
                    tracing::debug!("******  SLIDESHOW MODE *******");
                    if on_calendar::is_time_after_target(output.target_time, current_time) {
                        tracing::debug!("******  TIMER FIRED *******");
                        swww::set_wallpaper(output);
                        output.target_time = systemd_analyze::get_next_event(&output.oncalendar_string);
                    }
                },
                Mode::Oneshot => {
                    tracing::debug!("******  ONESHOT MODE *******");
                },
            }

            tracing::debug!("Done checking output");
        }

        ghetto_profiler.stop();

        thread::sleep(sleep_duration);
    }
}
