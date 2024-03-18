use std::thread;
use std::time::Duration;
use futures::executor::block_on;

mod dbus_server;

#[async_std::main]
async fn main() {
    println!("wallsd started");

    thread::spawn(|| {
        println!("Starting dbus server");
        let _ = block_on(dbus_server::run_server());
    });

    loop {
        println!("Listening for dbus events!");
        thread::sleep(Duration::from_secs(10));
    }
}
