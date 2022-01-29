import init, { greet } from "../../pkg/wasm_test.js";
init().then(() => {
  greet("WebAssembly");
});
