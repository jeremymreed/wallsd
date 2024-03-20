use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use futures::executor::block_on;

mod dbus_server;
mod swaymsg;
mod swww;
mod collection;

#[async_std::main]
async fn main() {
    println!("wallsd started");

    let (tx, rx) = mpsc::channel::<String>();

    let outputs = swaymsg::get_outputs();

    println!("Found outputs: {:?}", outputs);

    let mut collection: collection::Collection = collection::Collection {
        collection: Vec::new(),
    };

    collection.scan_collection(&String::from("/home/jeremyr/Pictures/Wallpapers"));

    println!("Number of wallpapers: {}", collection.collection.len());

    thread::spawn(|| {
        println!("Starting dbus server");
        let _ = block_on(dbus_server::run_server(tx, outputs, collection));
    });

    loop {
        println!("Listening for dbus events!");
        match rx.try_recv() {
            Ok(message) => println!("Got message: {}", message),
            Err(_) => println!("No message"),
        };
        thread::sleep(Duration::from_secs(10));
    }
}
