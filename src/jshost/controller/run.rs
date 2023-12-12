use std::{error::Error, future::pending};

use scorched::{log_this, LogData, LogImportance};
use zbus::ConnectionBuilder;

use crate::jshost::controller::interface::JsInterface;

pub async fn run() -> Result<(), Box<dyn Error>> {
    log_this(LogData {
        importance: LogImportance::Info,
        message: "Starting JsHost in controller mode.".to_string(),
    });

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
