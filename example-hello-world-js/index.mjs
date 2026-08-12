// @ts-check
import * as sloe from "./sloe.mjs";

const greeting = sloe.greet({ name: "world", result_origin: {} });
console.log(sloe.buf$span_slice(greeting.buf, greeting.span).join(""));
