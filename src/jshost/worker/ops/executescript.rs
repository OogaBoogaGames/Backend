use deno_core::JsRuntime;
use ipc_channel::platform::OsIpcSender;

use crate::jshost::controller::interface::{Message, Op};

pub fn executescript(
    msg: Message,
    script: String,
    (runtime, controller_tx): (&mut JsRuntime, OsIpcSender),
) -> bool {
    runtime
        .execute_script("__obg__.runtime.exec", script.into())
        .unwrap();

    let next = msg.next(Op::ExecuteComplete(Ok("".to_string())));
    controller_tx
        .clone()
        .send(&bincode::serialize(&next).unwrap()[..], vec![], vec![])
        .unwrap();

    true
}
