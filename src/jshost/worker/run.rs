use std::{
    error::Error,
    process,
    sync::{Arc, Mutex},
};

use deno_core::{JsRuntime, RuntimeOptions};
use ipc_channel::platform::{self, OsIpcChannel, OsIpcSender};
use scorched::{logf, set_log_prefix, LogData, LogImportance};

use crate::{
    jshost::{
        controller::interface::{Message, Op},
        worker::js::ext::oogabooga,
    },
    message_handler,
};

pub async fn run(name: String) -> Result<(), Box<dyn Error>> {
    let prefix = format!("Worker at \"{}\" (pid {}):", &name, process::id());
    set_log_prefix(&prefix);

    logf!(Info, "Starting JsHost in worker mode.");

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

    let mut runtime = JsRuntime::new(RuntimeOptions {
        extensions: vec![oogabooga::init_ops_and_esm()],
        ..Default::default()
    });

    loop {
        message_handler!(
            worker_rx.lock().unwrap(),
            Op::Init => |msg: Message | {
                let next = msg.next(Op::InitComplete);
                controller_tx
                    .clone()
                    .send(&bincode::serialize(&next).unwrap()[..], vec![], vec![])
                    .unwrap();
                },
            Op::ExecuteScript(script) => |msg: Message| {
                runtime
                    .execute_script("__obg__.runtime", script.into())
                    .unwrap();
                let next = msg.next(Op::ExecuteComplete(Ok("".to_string())));
                controller_tx
                    .clone()
                    .send(&bincode::serialize(&next).unwrap()[..], vec![], vec![])
                    .unwrap();
            }
        );
    }
}
