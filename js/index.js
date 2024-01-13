import { initSync, ask_deep_thought, initialize } from '../pkg/rustart.js';
import wasmData from '../pkg/rustart_bg.wasm'

console.log("Loading WASM")
initSync(wasmData);
console.log("WASM loaded, initializing")
initialize();
console.log("initialized")

document.querySelector('#askButton').addEventListener('click', async () => {
let input = document.querySelector('#question').value;
let result = await ask_deep_thought(input);
//   let result = await answer(input);
console.log(result);
});