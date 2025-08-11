mod server;
mod queue;
mod instance;
mod manager;

use std::sync::{Arc, Mutex};

use tokio;
use server::CBCAServer;
use queue::CBCAQueue;

use crate::server::CBCARoutineAddr;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let serv: CBCAServer = CBCAServer::spawn(
        CBCARoutineAddr::spawn("127.0.0.1".to_string(), "8686".to_string()),
        CBCARoutineAddr::spawn("127.0.0.1".to_string(), "8687".to_string()),
        CBCARoutineAddr::spawn("127.0.0.1".to_string(), "8688".to_string())
    )?;

    let _ = tokio::join!(
        biased;
        serv.run_routines()
    );

    Ok(())
}