import { threads } from 'wasm-feature-detect';
import * as Comlink from 'comlink';

var replInstance;
// Run wasm-bindgen exports (the Lurk REPL).
function runLurk({ Repl }) {
  replInstance = new Repl();
  return ({ textContent }) => {
    try {
      return replInstance.execute_lurk(textContent);
    } catch (err) {
      console.log('An error has occured, resetting engine state: ' + err);
      replInstance = new Repl();
      return JSON.stringify({iterations: 0, result: 'An error has occured, resetting engine state: ' + err});
    }
  };
}

async function initHandlers() {
  let [singleThread, multiThread] = await Promise.all([
    (async () => {
      const singleThread = await import('../pkg/index.js');
      await singleThread.default();
      return runLurk(singleThread);
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