import { initSync, PlotCanvas, initialize } from '../pkg/rustart.js';
import wasmData from '../pkg/rustart_bg.wasm'

console.log("Loading WASM")
initSync(wasmData);
console.log("WASM loaded, initializing")
initialize();
console.log("initialized")

// document.querySelector('#askButton').addEventListener('click', async () => {
//   let input = document.querySelector('#question').value;
//   let result = await ask_deep_thought(input);
//   console.log(result);
// });

const canvas = document.querySelector('#plot');
console.log("Found canvas: ", canvas);

let plot_canvas = new PlotCanvas(canvas);
console.log("Plot: ", plot_canvas);

plot_canvas.draw_line(40, 23, 100, 150);