use deno_core::JsRuntime;
use ipc_channel::platform::OsIpcSender;

use crate::jshost::controller::interface::{Message, Op};

pub fn init(msg: Message, (_, controller_tx): (&mut JsRuntime, OsIpcSender)) -> bool {
    let next = msg.next(Op::InitComplete);

    controller_tx
        .clone()
        .send(&bincode::serialize(&next).unwrap()[..], vec![], vec![])
        .unwrap();

    true
}
