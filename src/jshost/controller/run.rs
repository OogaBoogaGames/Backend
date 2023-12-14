use std::{error::Error, future::pending};

use scorched::{log_this, logf, LogData, LogImportance};
use zbus::ConnectionBuilder;

use crate::jshost::controller::interface::JsInterface;

pub async fn run() -> Result<(), Box<dyn Error>> {
    logf!(Info, "Starting JsHost in controller mode.");

    let greeter = JsInterface {
        workers: Default::default(),
    };

    let _conn = ConnectionBuilder::session()?
        .name("games.oogabooga.JsHost")?
        .serve_at("/games/oogabooga/JsHost", greeter)?
        .build()
        .await?;

    pending::<()>().await;

    Ok(())
}
