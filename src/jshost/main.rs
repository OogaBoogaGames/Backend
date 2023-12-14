use std::{env, error::Error};

use oogaboogagames_backend::jshost::{controller, worker};

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().skip(1).collect::<Vec<_>>();

    if let Some(arg) = args
        .iter()
        .filter(|arg| arg.starts_with("--worker="))
        .map(|arg| arg.trim_start_matches("--worker="))
        .filter(|s| !s.is_empty())
        .last()
    {
        return worker::run::run(arg.to_string()).await;
    } else {
        return controller::run::run().await;
    }
}
