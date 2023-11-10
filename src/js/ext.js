import {
  Game,
  GameState,
  GameStage,
} from "ext:oogabooga/node_modules/@oogaboogagames/game-core/dist/ext/GameBase.js";

import { Player } from "ext:oogabooga/node_modules/@oogaboogagames/game-core/dist/ext/Player.js";

import { LobbyStage } from "ext:oogabooga/node_modules/@oogaboogagames/game-core/dist/ext/LobbyStage.js";

let internalDeno = Deno.core;

globalThis.console = {
  log: (msg) => {
    internalDeno.print(msg + "\n");
    return msg;
  },
};

// Deny access to Deno APIs
delete Deno.core;

// Populate global context with the game APIs
globalThis.OogaBooga = {
  Game,
  GameState,
  LobbyStage,
  Player,
  GameStage,
};

console.log("Hello from OogaBooga!");
