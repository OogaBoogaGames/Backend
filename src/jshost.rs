mod js;

use std::{error::Error, future::pending};

use deno_core::{FastString, JsRuntime, RuntimeOptions};
use zbus::{dbus_interface, ConnectionBuilder};

use crate::js::ext::oogabooga;

struct Greeter {
    count: u64,
}

#[dbus_interface(name = "games.oogabooga.JsHost")]
impl Greeter {
    // Can be `async` as well.
    fn say_hello(&mut self, name: &str) -> String {
        self.count += 1;
        format!("Hello {}! I have been called {} times.", name, self.count)
    }
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let mut runtime = JsRuntime::new(RuntimeOptions {
        extensions: vec![oogabooga::init_ops_and_esm()],
        ..Default::default()
    });

    runtime
        .execute_script_static(
            "__obg__.preload",
            r#"

            // src/stages/stage1.ts
            var Game = globalThis.OogaBooga.Game;
            var GameStage = globalThis.OogaBooga.GameStage;
            var Player = globalThis.OogaBooga.Player;
            
            class Stage1 extends GameStage {
              constructor() {
                super(...arguments);
              }
              prompt = "What is your name?";
              responses = new Map;
              onstart(game) {
                console.log("Starting stage 1");
              }
              onresponse(game, player, response) {
                console.log("Received response from player", player);
              }
            }
            
            // src/stages/s
            var Game2 = globalThis.OogaBooga.Game;
            var GameState = globalThis.OogaBooga.GameState;
            var LobbyStage = globalThis.OogaBooga.LobbyStage;
            var Player2 = globalThis.OogaBooga.Player;
            var GameStage2 = globalThis.OogaBooga.GameStage;
            
            class ExampleGame extends Game2 {
              name = "Example Game";
              maxPlayers = 8;
              minPlayers = 2;
              stages = [
                new LobbyStage(this, (game) => {
                  game.stages[1].onstart(game);
                }),
                new Stage1(this, (game) => game.stages[2].onstart(game))
              ];
              start() {
                this.stages[0].onstart(this);
              }
              end() {
                throw new Error("Method not implemented.");
              }
              constructor() {
                super();
              }
            }

            "#,
        )
        .unwrap();

    let realm = runtime.main_realm();

    let isolate = runtime.v8_isolate();

    realm
        .execute_script(
            isolate,
            "__obg__.runtime",
            FastString::from_static("var game = new ExampleGame(); game.start();"),
        )
        .unwrap();

    let greeter = Greeter { count: 0 };
    let _conn = ConnectionBuilder::session()?
        .name("games.oogabooga.JsHost")?
        .serve_at("/games/oogabooga/JsHost", greeter)?
        .build()
        .await?;

    pending::<()>().await;

    Ok(())
}
