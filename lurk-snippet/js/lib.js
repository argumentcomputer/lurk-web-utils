import init from '../pkg/index.js';
import * as Comlink from 'comlink';
import { threads } from 'wasm-feature-detect';

var globalHandler;
export async function initHandlers() {
  if (await threads()) {
    console.log("WebAssembly threads supported.")
  } else {
    console.log("WebAssembly threads not supported!!")
    return null;
  }
  await init();
  // Create a separate thread from wasm-worker.js and get a proxy to its handlers.
  let handlers = await Comlink.wrap(
      new Worker(new URL('./wasm-worker.js', import.meta.url), {
        type: 'module'
      })
  ).handlers;
  globalHandler = handlers;
  return handlers;
}

export async function handleExpression(content) {
  if(!globalHandler) {
    await initHandlers(); // initialize it then
  }
  let out = await globalHandler({content});
  return out;
}

export async function handlerFunction() {
  if(!globalHandler) {
    await initHandlers(); // initialize it then
  }
  return globalHandler;
}