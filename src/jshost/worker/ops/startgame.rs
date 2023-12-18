use std::{env, fs::File, path::PathBuf};

use caveman::proto::Caveman::CavemanGameBundle;
use deno_core::{FastString, JsRuntime};
use ipc_channel::platform::OsIpcSender;
use protobuf::Message as _;
use scorched::{logf, LogData, LogExpect, LogImportance};

use crate::{
    backend::util::id::{GameId, OBGId},
    jshost::controller::interface::{Message, Op},
};

pub fn startgame(
    msg: Message,
    id: GameId,
    owner: OBGId,
    (runtime, controller_tx): (&mut JsRuntime, OsIpcSender),
) {
    runtime
        .execute_script(
            "__obg__.runtime.exec",
            FastString::from(format!("globalThis.target.start(\"{id}\", \"{owner}\");")),
        )
        .unwrap();

    let next = msg.next(Op::StartGameComplete(Ok(())));
    controller_tx
        .clone()
        .send(&bincode::serialize(&next).unwrap()[..], vec![], vec![])
        .unwrap();
}
