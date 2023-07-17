/**
 * The library **must** be built before running these tests.
 * Building first ensures that we are testing against the built wasm bindings.
 * Since we already have Rust tests for the core algorithm,
 * testing the wasm bindings is the **primary** goal of these specs.
 * Specifically, we want to make sure that strings are passed to and from the wasm code correctly.
 */
const { it, describe } = require("node:test");
const { strictEqual } = require("node:assert");
const { toCamelCase, toSnakeCase } = require("../dist/index");

const testSuites = [
  {
    name: "snake_case",
    fn: toSnakeCase,
    tests: [
      { input: "test lots", output: "test_lots" },
      { input: "Per Martin-Löf", output: "per_martin_löf" },
      { input: "Löwe 老虎 Léopard", output: "löwe_老虎_léopard" },
      { input: "y̆ummyYummy̆", output: "y̆ummy_yummy̆" },
      { input: "y̆ummy̆Yummy̆", output: "y̆ummy̆_yummy̆" },
    ],
  },
  {
    name: "camelCase",
    fn: toCamelCase,
    tests: [
      { input: "test lots", output: "testLots" },
      { input: "Per Martin-Löf", output: "perMartinLöf" },
      { input: "Löwe 老虎 Léopard", output: "löwe老虎Léopard" },
      { input: "y̆ummy_yummy̆", output: "y̆ummyYummy̆" },
      { input: "y̆ummy̆_yummy̆", output: "y̆ummy̆Yummy̆" },
    ],
  },
];

for (const { name, fn, tests } of testSuites) {
  describe(name, () => {
    for (const { input, output } of tests) {
      it(`input: ${input}, output: ${output}`, () => {
        strictEqual(fn(input), output);
      });
    }
  });
}
