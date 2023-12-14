use deno_core::extension;
use deno_core::op2;
use scorched::{logf, LogData, LogImportance};
use serde::{Deserialize, Serialize};

#[op2(fast)]
fn is_prod() -> bool {
    !cfg!(debug_assertions)
}
#[derive(Serialize, Deserialize)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

#[op2]
fn op_log(#[serde] level: LogLevel, #[string] msg: String) {
    match level {
        LogLevel::Debug => logf!(Debug, "JS: {}", msg),
        LogLevel::Info => logf!(Info, "JS: {}", msg),
        LogLevel::Warning => logf!(Warning, "JS: {}", msg),
        LogLevel::Error => logf!(Error, "JS: {}", msg),
    };
}

#[op2(fast)]
fn op_get_players() -> Result<f64, deno_core::error::AnyError> {
    Ok(100.0)
}

extension!(
    oogabooga,
    ops = [op_get_players, op_log, is_prod],
    esm_entry_point = "ext:oogabooga/src/jshost/worker/js/ext.js",
    esm = [
        "src/jshost/worker/js/ext.js",
        "node_modules/@oogaboogagames/game-core/dist/ext/GameBase.js",
        "node_modules/@oogaboogagames/game-core/dist/ext/Player.js",
        "node_modules/@oogaboogagames/game-core/dist/ext/LobbyStage.js",
    ],
);
