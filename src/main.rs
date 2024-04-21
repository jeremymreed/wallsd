use std::{thread, time::Duration};
use async_std::channel::unbounded;
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

    let (sender_dbus, receiver_main) = unbounded::<InternalCommand>();

    thread::spawn(|| {
        tracing::info!("Starting dbus server");
        let _ = block_on(dbus_server::run_server(sender_dbus));
    });

    let sleep_duration = Duration::from_secs(1);

    tracing::info!("Starting main loop");
    loop {
        let ghetto_profiler = profiler::Profiler::start();

        let current_time = Local::now();

        tracing::trace!("Checking for dbus events!");
        match receiver_main.try_recv() {
            Ok(message) => {
                let response = match executor.poll_dbus_messages(message) {
                    Ok(response) => response,
                    Err(response) => response,
                };
            },
            Err(_) => tracing::debug!("No message"),
        };

        executor.check_outputs(current_time);

        ghetto_profiler.stop();

        thread::sleep(sleep_duration);
    }
}
