use std::sync::mpsc::Sender;
use zbus::{connection::Builder, interface, Result};
use zvariant::Type;
use serde::{Deserialize, Serialize};
use event_listener::{Event, Listener};
use crate::command;
use crate::status::Status;
use crate::collection;
use crate::swww;

#[derive(Serialize, Deserialize, Type, Debug)]
struct Point {
    x: i32,
    y: i32,
}

struct DbusServer {
    tx: Sender<command::InternalCommand>,
    done: Event,
}

#[interface(name = "com.thetechforest.WallsD")]
impl DbusServer {
    async fn set_output_mode(&self, command: command::SetOutputModeCommand) -> command::SetOutputModeResponse {
        tracing::debug!("set_output_mode called with command: {:#?}", command);

        self.tx.send(command::InternalCommand::SetOutputModeCommand(command)).unwrap();

        command::SetOutputModeResponse {
            status: Status::Success,
            error: "".to_string(),
        }
    }
}

pub async fn run_server(tx: Sender<command::InternalCommand>) -> Result<()> {

    let dbus_server = DbusServer {
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
