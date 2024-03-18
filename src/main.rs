use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use futures::executor::block_on;

mod dbus_server;

#[async_std::main]
async fn main() {
    println!("wallsd started");

    let (tx, rx) = mpsc::channel::<String>();

    thread::spawn(|| {
        println!("Starting dbus server");
        let _ = block_on(dbus_server::run_server(tx));
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
