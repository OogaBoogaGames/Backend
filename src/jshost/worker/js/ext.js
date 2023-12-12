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

  function argsToMessage(...args) {
    return args
      .map((arg) =>
        JSON.stringify(arg, (_k, v) =>
          typeof v == "function" ? v.toString() : v,
        ),
      )
      .join(" ");
  }

  // Populate global context with the console API
  globalThis.console = new Proxy(
    {
      log: (...args) => {
        core.print(`[out]: ${argsToMessage(...args)}\n`, false);
      },
      error: (...args) => {
        core.print(`[err]: ${argsToMessage(...args)}\n`, true);
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
    },
  );

  const hideFn =
    (_0) =>
    (..._1) =>
      _0(..._1);

  // Populate global context with the Games object
  // Note: intentionally polluting the global context
  Object.defineProperty(globalThis, "Games", {
    value: (() => {
      const securedObject = {};

      return Object.freeze({
        set: hideFn((key, value, secretKey) => {
          securedObject[key] = {
            value: value,
            secretKey: secretKey,
          };
        }),
        get: hideFn((key, secretKey) => {
          if (
            key in securedObject &&
            securedObject[key].secretKey === secretKey
          ) {
            return securedObject[key].value;
          } else {
            throw new Error("Unauthorized access");
          }
        }),
        query: hideFn((key) => {
          return key in securedObject;
        }),
        delete: hideFn((key, secretKey) => {
          if (
            key in securedObject &&
            securedObject[key].secretKey === secretKey
          ) {
            delete securedObject[key];
          } else {
            throw new Error("Unauthorized access");
          }
        }),
        securedObject: hideFn(() => {
          if (isprod) {
            throw new Error("Unauthorized access");
          } else {
            return securedObject;
          }
        }),
      });
    })(),
    writable: false,
    configurable: false,
    enumerable: true,
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
