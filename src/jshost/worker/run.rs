use std::{
    error::Error,
    sync::{Arc, Mutex},
};

use ipc_channel::platform::{self, OsIpcChannel, OsIpcSender};
use scorched::{LogData, LogImportance};

use crate::jshost::controller::interface::{Message, Op};

use super::log::worker_log;

pub async fn run(name: String) -> Result<(), Box<dyn Error>> {
    worker_log(
        LogData {
            importance: LogImportance::Info,
            message: "Starting JsHost in worker mode.".to_string(),
        },
        Some(&name),
    );

    let (controller_tx, worker_tx, worker_rx) = {
        let (tx, rx) = platform::channel()?;
        (
            OsIpcSender::connect(name.to_string())?,
            tx,
            Arc::new(Mutex::new(rx)),
        )
    };

    controller_tx
        .send(&[], vec![OsIpcChannel::Sender(worker_tx.clone())], vec![])
        .unwrap();

    loop {
        match worker_rx.lock().unwrap().recv() {
            Ok(recv) => {
                if let Ok(msg) = bincode::deserialize::<Message>(&recv.0) {
                    println!("slave received data: {:?}", msg);
                    match msg.op() {
                        Op::Init => {
                            let next = msg.next(Op::InitComplete);
                            controller_tx
                                .clone()
                                .send(&bincode::serialize(&next).unwrap()[..], vec![], vec![])
                                .unwrap();
                        }

                        _ => {
                            println!("slave received data: {:?}", msg);
                        }
                    }
                } else {
                    println!("womp womp")
                }
            }
            _ => {}
        }
    }
}