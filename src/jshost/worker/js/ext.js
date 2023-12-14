import {
  Game,
  GameState,
  GameStage,
} from "ext:oogabooga/node_modules/@oogaboogagames/game-core/dist/ext/GameBase.js";

import { Player } from "ext:oogabooga/node_modules/@oogaboogagames/game-core/dist/ext/Player.js";

import { LobbyStage } from "ext:oogabooga/node_modules/@oogaboogagames/game-core/dist/ext/LobbyStage.js";

((globalThis) => {
  const core = Deno.core;

  const isprod = core.ops.is_prod();
  const op_log = core.ops.op_log;

  function argsToMessage(...args) {
    return args
      .map((arg) =>
        JSON.stringify(arg, (_k, v) =>
          typeof v == "function" ? v.toString() : v
        )
      )
      .join(" ");
  }

  // Populate global context with the console API
  Object.defineProperty(globalThis, "console", {
    value: new Proxy(
      {
        log: (...args) => {
          op_log("Info", argsToMessage(...args));
        },
        error: (...args) => {
          op_log("Error", argsToMessage(...args));
        },
      },
      {
        get: function (target, prop) {
          if (isprod) {
            throw new Error(`Cannot access console in production`);
          } else {
            return target[prop];
          }
        },
        set: function (target, prop, value) {
          if (isprod) {
            throw new Error(`Cannot access console in production`);
          } else {
            return target[prop];
          }
        },
      }
    ),
  });

  // Populate global context with the game APIs
  // Note: intentionally polluting the global context
  Object.defineProperty(globalThis, "OogaBooga", {
    value: Object.freeze({
      Game,
      GameState,
      LobbyStage,
      Player,
      GameStage,
    }),
    writable: false,
    configurable: false,
    enumerable: true,
  });
})(globalThis);

delete Deno.core;
delete Deno.__op__;
