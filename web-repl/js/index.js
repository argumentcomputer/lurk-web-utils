import css from 'xterm/css/xterm.css';
import 'xterm/lib/xterm.js';
import("../pkg/index.js").catch(console.error).then(wasm => {
  // Main js code
  console.log("Welcome to the Lurk web REPL");
});
