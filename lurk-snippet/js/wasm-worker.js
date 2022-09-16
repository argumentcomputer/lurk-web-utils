import { threads } from 'wasm-feature-detect';
import * as Comlink from 'comlink';

var replInstance;
// Wrap wasm-bindgen exports (the `generate` function) to add time measurement.
function wrapExports({ Repl }) {
  replInstance = new Repl();
  return ({ textContent }) => {
    try {
      return replInstance.execute_lurk(textContent);
    } catch (err) {
      console.log('An error has occured, resetting Lurk Repl state: ' + err);
      replInstance = new Repl();
      return JSON.stringify({iterations: 0, result: 'An error has occured, resetting repl state: ' + err});
    }
  };
}

async function initHandlers() {
  let [singleThread, multiThread] = await Promise.all([
    (async () => {
      const singleThread = await import('../pkg/index.js');
      await singleThread.default();
      return wrapExports(singleThread);
    })(),
    (async () => {
      // If threads are unsupported in this browser, skip this handler.
      if (!(await threads())) return;
      const multiThread = await import(
        '../pkg/index.js'
      );
      await multiThread.default();
      await multiThread.initThreadPool(navigator.hardwareConcurrency);
      return wrapExports(multiThread);
    })()
  ]);

  return Comlink.proxy({
    singleThread,
    supportsThreads: !!multiThread,
    multiThread
  });
}

Comlink.expose({
  handlers: initHandlers()
});