import * as wasm from "gcalc";

let option = wasm.default_option();
option.prob = 0.1;
option.count = 10;
option.cost = 100
option.prob_precision = 2;
let csv = wasm.calculate("range", option);
document.querySelector("#content").textContent = csv;

export let csv_value = csv;
