use deno_core::extension;
use deno_core::op2;

#[op2(fast)]
fn is_prod() -> bool {
    !cfg!(debug_assertions)
}

#[op2(fast)]
fn op_get_players() -> Result<f64, deno_core::error::AnyError> {
    Ok(100.0)
}

extension!(
    oogabooga,
    ops = [op_get_players, is_prod],
    esm_entry_point = "ext:oogabooga/src/jshost/worker/js/ext.js",
    esm = [
        "src/jshost/worker/js/ext.js",
        "node_modules/@oogaboogagames/game-core/dist/ext/GameBase.js",
        "node_modules/@oogaboogagames/game-core/dist/ext/Player.js",
        "node_modules/@oogaboogagames/game-core/dist/ext/LobbyStage.js",
    ],
);
