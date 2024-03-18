use zbus::{connection::Builder, interface, Connection, Result};
use event_listener::{Event, Listener};

struct DbusServer {
    done: Event,
}

#[interface(name = "com.thetechforest.WallsD1")]
impl DbusServer {
    async fn say_hello(&self, name: &str) -> String {
        println!("say_hello called with name: {}", name);
        format!("Hello, {}!", name)
    }
}

pub async fn run_server() -> Result<()> {

    let dbus_server = DbusServer {
        done: event_listener::Event::new(),
    };

    let done_listener = dbus_server.done.listen();

    let _connection = Builder::session()?
        .name("com.thetechforest.WallsD")?
        .serve_at("/com/thetechforest/WallsD", dbus_server)?
        .build()
        .await?;

    done_listener.wait();

    Ok(())
}