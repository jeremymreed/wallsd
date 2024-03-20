use std::sync::mpsc::Sender;
use zbus::{connection::Builder, interface, Result};
use zvariant::Type;
use serde::{Deserialize, Serialize};
use event_listener::{Event, Listener};
use crate::collection;
use crate::swww;

#[derive(Serialize, Deserialize, Type, Debug)]
struct Point {
    x: i32,
    y: i32,
}

struct DbusServer {
    outputs: Vec<String>,
    collection: collection::Collection,
    tx: Sender<String>,
    done: Event,
}

#[interface(name = "com.thetechforest.WallsD")]
impl DbusServer {
    async fn say_hello(&self, name: &str) -> String {
        self.tx.send(String::from("Foo Bar")).unwrap();

        println!("say_hello called with name: {}", name);
        format!("Hello, {}!", name)
    }

    async fn test(&self, point: Point) {
        println!("test called with point: {:#?}", point);
    }

    async fn set_wallpaper(&self) {
        swww::set_wallpapers(&self.collection, &self.outputs);
    }
}

pub async fn run_server(tx: Sender<String>, outputs: Vec<String>, collection: collection::Collection) -> Result<()> {

    let dbus_server = DbusServer {
        outputs,
        collection,
        tx,
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
