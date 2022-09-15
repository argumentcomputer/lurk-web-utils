import css from 'xterm/css/xterm.css';
import 'xterm/lib/xterm.js';
import init, { initThreadPool /* ... */ } from '../pkg/index.js';

(async function() {
  await init();
  initThreadPool(navigator.hardwareConcurrency);

  console.log('Thread pool initiated');

})();
