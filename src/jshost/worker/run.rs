use std::{
    env,
    error::Error,
    fs::File,
    path::PathBuf,
    process,
    sync::{Arc, Mutex},
};

use caveman::proto::Caveman::CavemanGameBundle;
use deno_core::{FastString, JsRuntime, RuntimeOptions};
use ipc_channel::platform::{self, OsIpcChannel, OsIpcSender};
use protobuf::Message as _;
use scorched::{logf, set_log_prefix, LogData, LogExpect, LogImportance};

use crate::{
    jshost::{
        controller::{
            self,
            interface::{Message, Op},
        },
        worker::{js::ext::oogabooga, ops},
    },
    message_handler,
};

pub async fn run(name: String) -> Result<(), Box<dyn Error>> {
    let prefix = format!("Worker at \"{}\" (pid {}):", &name, process::id());
    set_log_prefix(&prefix);

    env::var("GAMES_PATH").unwrap_or_else(|_| {
        env::set_var("GAMES_PATH", "games/");
        "games/".to_string()
    });

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
            Op::Init => |msg: Message | ops::init::init(msg, (&mut runtime, controller_tx.clone())),
            Op::LoadGame(id) => |msg: Message| ops::loadgame::loadgame(msg, id, (&mut runtime, controller_tx.clone())),
            Op::ExecuteScript(script) => |msg: Message| ops::executescript::executescript(msg, script, (&mut runtime, controller_tx.clone()))
        );
    }
}
