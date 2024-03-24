use std::sync::mpsc;
use std::{thread, time::{Duration, Instant}};
use chrono::Local;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use futures::executor::block_on;
use crate::output::Output;

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
    tracing::info!("wallsd started");

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    let (tx, rx) = mpsc::channel::<String>();

    let mut outputs = swaymsg::get_outputs();

    tracing::debug!("Found outputs: {:#?}", outputs);

    let mut collection: collection::Collection = collection::Collection {
        collection: Vec::new(),
    };

    collection.scan_collection(&String::from("/home/jeremyr/Pictures/Wallpapers"));

    tracing::debug!("Number of wallpapers: {}", collection.collection.len());

    //for index in 0..outputs.len() {
    for output in &mut outputs {
        output.images = collection.collection.clone();
    }

    //tracing::trace!("Outputs with images: {:#?}", outputs);

    /*
    thread::spawn(|| {
        debug!("Starting dbus server");
        let _ = block_on(dbus_server::run_server(tx));
    });
    */

    let sleep_duration = Duration::from_secs(1);

    let oncalendar_string: String = "*-*-* *:0/2".to_string();
    tracing::debug!("oncalendar_string: {:?}", oncalendar_string);

    let mut target  = systemd_analyze::get_next_event(&oncalendar_string);

    loop {
        let now = Instant::now();

        let current_time = Local::now();

        tracing::trace!("Checking for dbus events!");
        match rx.try_recv() {
            Ok(message) => tracing::debug!("Got message: {}", message),
            Err(_) => tracing::debug!("No message"),
        };

        // Check to see if the timer should be fired.
        if on_calendar::is_time_after_target(target, current_time) {
            tracing::debug!("******  TIMER FIRED *******");
            for output in &outputs {
                swww::set_wallpaper(output);
            }
            target = systemd_analyze::get_next_event(&oncalendar_string);
        }

        let elapsed = now.elapsed();
        tracing::trace!("Elapsed time: {:#?}\n\n", elapsed);


        thread::sleep(sleep_duration);
    }
}
