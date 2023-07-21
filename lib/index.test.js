/**
 * The library **must** be built before running these tests.
 * Building first ensures that we are testing against the built wasm bindings.
 * Since we already have Rust tests for the core algorithm,
 * testing the wasm bindings is the **primary** goal of these specs.
 * Specifically, we want to make sure that strings are passed to and from the wasm code correctly.
 */
const { it, describe } = require("node:test");
const { strictEqual } = require("node:assert");
const {
  toCamelCase,
  toSnakeCase,
  toPascalCase,
  toTitleCase,
  toKebabCase,
  toSentenceCase,
  toScreamingSnakeCase,
} = require("../dist/index");

const testSuites = [
  {
    name: "snake_case",
    fn: toSnakeCase,
    tests: [
      { input: "test lots", output: "test_lots" },
      { input: "Per Martin-Löf", output: "per_martin_löf" },
      { input: "Löwe 老虎 Léopard", output: "löwe_老虎_léopard" },
      { input: "y̆ummyYummy̆", output: "y̆ummy_yummy̆" },
      { input: "y̆ummy̆Yummy̆", output: "y̆ummy̆yummy̆" },
    ],
  },
  {
    name: "SCREAMING_SNAKE_CASE",
    fn: toScreamingSnakeCase,
    tests: [
      { input: "test lots", output: "TEST_LOTS" },
      { input: "Per Martin-Löf", output: "PER_MARTIN_LÖF" },
      { input: "Löwe 老虎 Léopard", output: "LÖWE_老虎_LÉOPARD" },
      { input: "y̆ummyYummy̆", output: "Y̆UMMY_YUMMY̆" },
      { input: "y̆ummy̆Yummy̆", output: "Y̆UMMY̆YUMMY̆" },
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
  {
    name: "PascalCase",
    fn: toPascalCase,
    tests: [
      { input: "test lots", output: "TestLots" },
      { input: "Per Martin-Löf", output: "PerMartinLöf" },
      { input: "Löwe 老虎 Léopard", output: "Löwe老虎Léopard" },
      { input: "y̆ummyYummy̆", output: "Y̆ummyYummy̆" },
      { input: "y̆ummy̆Yummy̆", output: "Y̆ummy̆yummy̆" },
    ],
  },
  {
    name: "Title Case",
    fn: toTitleCase,
    tests: [
      { input: "test lots", output: "Test Lots" },
      { input: "Per Martin-Löf", output: "Per Martin Löf" },
      { input: "Löwe 老虎 Léopard", output: "Löwe 老虎 Léopard" },
      { input: "y̆ummyYummy̆", output: "Y̆ummy Yummy̆" },
      { input: "y̆ummy̆Yummy̆", output: "Y̆ummy̆yummy̆" },
    ],
  },
  {
    name: "kebab-case",
    fn: toKebabCase,
    tests: [
      { input: "test lots", output: "test-lots" },
      { input: "Per Martin-Löf", output: "per-martin-löf" },
      { input: "Löwe 老虎 Léopard", output: "löwe-老虎-léopard" },
      { input: "y̆ummyYummy̆", output: "y̆ummy-yummy̆" },
      { input: "y̆ummy̆Yummy̆", output: "y̆ummy̆yummy̆" },
    ],
  },
  {
    name: "Sentence case",
    fn: toSentenceCase,
    tests: [
      { input: "test lots", output: "Test lots" },
      { input: "Per Martin-Löf", output: "Per martin löf" },
      { input: "Löwe 老虎 Léopard", output: "Löwe 老虎 léopard" },
      { input: "y̆ummyYummy̆", output: "Y̆ummy yummy̆" },
      { input: "y̆ummy̆Yummy̆", output: "Y̆ummy̆yummy̆" },
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
