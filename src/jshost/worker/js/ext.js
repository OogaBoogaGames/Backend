import {
  Game,
  GameStage,
} from "ext:oogabooga/node_modules/@oogaboogagames/game-core/dist/ext/GameBase.js";

import { obg_main } from "ext:oogabooga/node_modules/@oogaboogagames/game-core/dist/ext/Decorators.js";

import { ErrorStage } from "ext:oogabooga/node_modules/@oogaboogagames/game-core/dist/ext/ErrorStage.js";

import { Player } from "ext:oogabooga/node_modules/@oogaboogagames/game-core/dist/ext/Player.js";

import { LobbyStage } from "ext:oogabooga/node_modules/@oogaboogagames/game-core/dist/ext/LobbyStage.js";

((globalThis) => {
  const core = Deno.core;

  const isprod = core.ops.is_prod();
  const op_log = core.ops.op_log;

  function argsToMessage(...args) {
    const seen = new WeakSet();
    return args
      .map((arg) =>
        JSON.stringify(arg, (_k, v) => {
          if (typeof value === "object" && value !== null) {
            if (seen.has(v)) {
              return "[Circular]";
            }
            seen.add(v);
          }
          return typeof v == "function" ? v.toString() : v;
        })
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
      LobbyStage,
      Player,
      GameStage,
      ErrorStage,
    }),
    writable: false,
    configurable: false,
    enumerable: true,
  });

  let timers = {
    setTimeout: function (callback, delay) {
      return core.queueTimer(core.getTimerDepth() + 1, false, delay, callback);
    },
    setInterval: function (callback, delay) {
      return core.queueTimer(core.getTimerDepth() + 1, true, delay, callback);
    },
    clearTimeout: function (id) {
      core.cancelTimer(id);
    },
    clearInterval: function (id) {
      core.cancelTimer(id);
    },
    unrefTimer: function (id) {
      core.unrefTimer(id);
    },
    refTimer: function (id) {
      core.refTimer(id);
    },
  };

  Object.keys(timers).forEach((key) => {
    globalThis[key] = timers[key];
  });
})(globalThis);

delete Deno.core;
delete Deno.__op__;
