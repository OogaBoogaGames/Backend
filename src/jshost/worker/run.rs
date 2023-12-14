use std::{
    error::Error,
    sync::{Arc, Mutex},
};

use ipc_channel::platform::{self, OsIpcChannel, OsIpcSender};
use scorched::{LogData, LogImportance};

use crate::{
    jshost::controller::interface::{Message, Op},
    message_handler,
};

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
        let (worker_tx, worker_rx) = platform::channel()?;
        (
            OsIpcSender::connect(name.to_string())?,
            worker_tx,
            Arc::new(Mutex::new(worker_rx)),
        )
    };

    controller_tx
        .send(&[], vec![OsIpcChannel::Sender(worker_tx.clone())], vec![])
        .unwrap();

    loop {
        message_handler!(worker_rx.lock().unwrap(), Op::Init => |msg: Message | {
            let next = msg.next(Op::InitComplete);
            controller_tx
                .clone()
                .send(&bincode::serialize(&next).unwrap()[..], vec![], vec![])
                .unwrap();
        });
    }
}
