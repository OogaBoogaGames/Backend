use std::{env, fs::File, path::PathBuf};

use caveman::proto::Caveman::CavemanGameBundle;
use deno_core::{FastString, JsRuntime};
use ipc_channel::platform::OsIpcSender;
use protobuf::Message as _;
use scorched::{logf, LogData, LogExpect, LogImportance};

use crate::{
    backend::util::id::OBGId,
    jshost::controller::interface::{Message, Op},
};

pub fn loadgame(
    msg: Message,
    id: OBGId,
    (runtime, controller_tx): (&mut JsRuntime, OsIpcSender),
) -> bool {
    let next = msg.next(Op::LoadGameComplete(Ok(())));

    let games_path: PathBuf = env::var("GAMES_PATH")
        .unwrap_or_else(|_| "games/".to_string())
        .into();

    let file = File::open(games_path.join(format!("{}.obg", id)))
        .log_expect(LogImportance::Error, "Failed to open game file.");

    let game = CavemanGameBundle::parse_from_reader(&mut &file)
        .log_expect(LogImportance::Error, "Failed to parse game file.");

    let code = String::from_utf8(game.runtime).unwrap();

    logf!(Info, "Loading game {}.", id);

    runtime
        .execute_script("__obg__.runtime.load", FastString::from(code))
        .unwrap();

    logf!(Info, "Finished loading game {}.", id);

    controller_tx
        .clone()
        .send(&bincode::serialize(&next).unwrap()[..], vec![], vec![])
        .unwrap();

    true
}
