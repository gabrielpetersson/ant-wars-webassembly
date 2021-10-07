import * as wasm from "../pkg/ant_wars_bg.wasm";

let result = wasm.add(2, 8);
wasm.create_element();
console.log(result);
