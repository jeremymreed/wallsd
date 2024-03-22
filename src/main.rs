use std::sync::mpsc;
use std::{thread, time::{Duration, Instant}};
use chrono::Local;
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
    println!("wallsd started");

    let (tx, rx) = mpsc::channel::<String>();

    let mut outputs = swaymsg::get_outputs();

    println!("Found outputs: {:#?}", outputs);

    let mut collection: collection::Collection = collection::Collection {
        collection: Vec::new(),
    };

    collection.scan_collection(&String::from("/home/jeremyr/Pictures/Wallpapers"));

    println!("Number of wallpapers: {}", collection.collection.len());

    //for index in 0..outputs.len() {
    for output in &mut outputs {
        output.images = collection.collection.clone();
    }

    println!("Outputs with images: {:#?}", outputs);

    /*
    thread::spawn(|| {
        println!("Starting dbus server");
        let _ = block_on(dbus_server::run_server(tx));
    });
    */

    let sleep_duration = Duration::from_secs(1);

    let oncalendar_string: String = "*-*-* *:0/2".to_string();
    println!("oncalendar_string: {:?}", oncalendar_string);

    let mut target  = systemd_analyze::get_next_event(&oncalendar_string);

    loop {
        let now = Instant::now();

        let current_time = Local::now();

        println!("Checking for dbus events!");
        match rx.try_recv() {
            Ok(message) => println!("Got message: {}", message),
            Err(_) => println!("No message"),
        };

        // Check to see if the timer should be fired.
        if on_calendar::is_time_after_target(target, current_time) {
            println!("******  TIMER FIRED *******");
            for output in &outputs {
                swww::set_wallpaper(output);
            }
            target = systemd_analyze::get_next_event(&oncalendar_string);
        }

        let elapsed = now.elapsed();
        println!("Elapsed time: {:#?}", elapsed);
        println!("\n");

        thread::sleep(sleep_duration);
    }
}
