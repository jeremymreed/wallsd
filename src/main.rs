mod dbus_server;

#[async_std::main]
async fn main() {
    println!("wallsd started");

    dbus_server::run_server().await;
}
