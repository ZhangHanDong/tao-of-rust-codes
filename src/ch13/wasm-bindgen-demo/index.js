
const js = import("./wasm_bindgen_demo");

js.then(js => {
  js.greet("World!");
});
