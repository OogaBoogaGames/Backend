use std::{env, error::Error};

use oogaboogagames_backend::jshost::{controller, worker};

// fn js_runtime_thread(rx: mpsc::Receiver<JsRuntimeMessage>) {
//     let mut runtime = JsRuntime::new(RuntimeOptions {
//         extensions: vec![oogabooga::init_ops_and_esm()],
//         ..Default::default()
//     });

//     for message in rx {
//         match message {
//             JsRuntimeMessage::ExecuteScript(script) => {
//                 runtime
//                     .execute_script("__obg__.runtime", FastString::from(script))
//                     .unwrap();
//             } // Handle other messages as needed
//         }
//     }
// }

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
