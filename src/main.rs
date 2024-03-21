use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use futures::executor::block_on;
use crate::output::Output;

mod command;
mod mode;
mod output;
mod resolution;
mod dbus_server;
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

    let mut seconds:u32 = 0;

    loop {
        println!("Listening for dbus events!");
        match rx.try_recv() {
            Ok(message) => println!("Got message: {}", message),
            Err(_) => println!("No message"),
        };

        println!("seconds: {}", seconds);

        if seconds >= 60 {
            for output in &outputs {
                swww::set_wallpaper(&output.clone());
            }
            seconds = 0;
        }

        thread::sleep(Duration::from_secs(1));
        seconds += 1;
    }
}
