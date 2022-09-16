import init from '../pkg/index.js';
import * as Comlink from 'comlink';

var originalContents;

(async function() {
  await init();
  originalContents = document.getElementById("lurkcode").textContent;
  HighlightLisp.highlight_auto();
  HighlightLisp.paren_match();
  
  // Create a separate thread from wasm-worker.js and get a proxy to its handlers.
  let handlers = await Comlink.wrap(
    new Worker(new URL('./wasm-worker.js', import.meta.url), {
      type: 'module'
    })
  ).handlers;

  console.log('Worker Threads LOADED');
  var output_container = document.getElementById("lurk-code-component-controls");
  output_container.style.display = "block";

  // let handler = handlers['singleThread'];
  let handler = handlers['multiThread'];
  // If handler doesn't exist, it's not supported.
  if (!handler) return;

  var btn = document.getElementById('run');
  btn.onclick = async function (e) {
      var output_container = document.getElementById("output-container");
      output_container.style.display = "block";
      var lurkcode = document.getElementById("lurkcode");        
      var output = document.getElementById("output");
      try {
          output.textContent ="processing... ";
          let textContent = lurkcode.textContent;
          let out = await handler({textContent});
          //var out = module.execute_lurk(lurkcode.textContent);
          var outObj = JSON.parse(out);
          output.textContent = "Iterations: " + outObj.iterations + "\nResult: " + outObj.result;
      } catch (error) {
          console.log(error);
          output.textContent ="Iterations: 0 \nResult: ERROR: " + error;
          return false;
      }
      
      return false;
  };
  var resetBtn = document.getElementById('reset');
  resetBtn.onclick = function (e) {
      document.getElementById("lurkcode").textContent = originalContents;
      HighlightLisp.highlight_auto();
      HighlightLisp.paren_match();
      var output_container = document.getElementById("output-container");
      output_container.style.display = "none";
  }

  document.addEventListener("update-progress", (event) => {
      var output = document.getElementById("output");
      output.textContent = event.detail.message();
  });

  /*
  setupBtn('singleThread');
  if (await handlers.supportsThreads) {
    setupBtn('multiThread');
  }
  */
})();
