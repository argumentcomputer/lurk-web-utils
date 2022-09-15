import css from 'xterm/css/xterm.css';
import {Terminal} from 'xterm/lib/xterm.js';
import { FitAddon } from 'xterm-addon-fit';
import init from '../pkg/index.js';
import * as Comlink from 'comlink';

(async function() {
  await init();
  //await initThreadPool(navigator.hardwareConcurrency);

  // Create a separate thread from wasm-worker.js and get a proxy to its handlers.
  let handlers = await Comlink.wrap(
    new Worker(new URL('./wasm-worker.js', import.meta.url), {
      type: 'module'
    })
  ).handlers;

  // let handler = handlers['singleThread'];
  let handler = handlers['multiThread'];
  // If handler doesn't exist, it's not supported.
  if (!handler) return;

  const term = new Terminal();
  const fitAddon = new FitAddon();
  term.loadAddon(fitAddon);
  
  /* You can make your terminals colorful :) */
  term.setOption("theme", {
    background: "#202B33",
    foreground: "#F5F8FA"
  });
  term.open(document.getElementById('lurk-terminal'));
  fitAddon.fit();

  var shellprompt = '\u001b[32mlurk> \u001b[37m';
  term.prompt = function () {
    term.write('\r\n' + shellprompt);
  };

  term.writeln('Welcome to Lurk REPL');
  // term.writeln('');
  term.prompt();
  term.setOption('cursorBlink', true);

  var entries = [];
  var currPos = 0;
  var cmd = '';

  term.onData(async e => {
    switch (e) {
      case '\u0003': // Ctrl+C
        term.write('^C');
        prompt(term);
        break;
      case '\r': // Enter
        if(cmd === 'clear') {
          cmd = '';
          term.clear();
          term.prompt();
          return;
        } else if (cmd == '') {
          term.prompt();
          return;
        }
        try {
          entries.push(cmd);
          currPos = entries.length;
          term.writeln('');
          //term.writeln("processing... ");
          let out = await handler({textContent: cmd});
          var outObj = JSON.parse(out);
          term.writeln("Iterations: " + outObj.iterations);
          term.writeln("Result: " + outObj.result);
          cmd = '';
          term.prompt();
        } catch (error) {
            console.log(error);
            term.writeln("Iterations: 0");
            term.writeln("Result: ERROR: " + error);
            return;
        }
        break;
      case '\u007F': // Backspace (DEL)
        // Do not delete the prompt
        if (term._core.buffer.x > 6) {
          term.write('\b \b');
          if (cmd.length > 0) {
            cmd = cmd.substr(0, cmd.length - 1);
          }
        }
        break;
      case '\x1B[A': // up arrow
        console.log('up arrow');
        if (entries.length > 0) {
          if (currPos > 0) {
            currPos -= 1;
          }
          cmd = entries[currPos];
          term.write('\x1b[2K\r'+shellprompt + cmd);
        }
        break;
      case '\x1B[B': // down arrow
        console.log('down arrow');
        currPos += 1;
        if (currPos === entries.length || entries.length === 0) {
          currPos -= 1;
          cmd = '';
        } else {
          cmd = entries[currPos];
        }
        term.write('\x1b[2K\r'+shellprompt + cmd);
        break;
      default: // Print all other characters for demo
        if (e >= String.fromCharCode(0x20) && e <= String.fromCharCode(0x7E) || e >= '\u00a0') {
          cmd += e;
          term.write(e);
        }
    }
  });
  console.log('Thread pool initiated');
})();
