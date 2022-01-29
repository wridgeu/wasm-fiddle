(async () => {
  const wasm_test = await import("wasm-test");
  wasm_test.greet("WebAssembly");
})();
