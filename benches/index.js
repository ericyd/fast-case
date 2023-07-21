// Credit: https://github.com/aarondcohen/benchmark-guid
const Benchmarkify = require("benchmarkify");

const {
  toSnakeCase,
  toCamelCase,
  toSentenceCase,
  toTitleCase,
  toPascalCase,
  toKebabCase,
  toScreamingSnakeCase,
} = require("../dist/index");

const {
  camelCase,
  capitalCase,
  constantCase,
  paramCase,
  pascalCase,
  sentenceCase,
  snakeCase,
} = require("change-case");

// the overhead of copying the string is a substantial part of the runtime for fast-case.
// Long strings perform better with fast-case because the algorithm is faster which offsets the time to copy the string.
// Short strings perform better with change-case because the algorithm can't make up the speed lost by copying the string
const longString = "SomeWhat varied test-string WITH_MULTIPLE wordCasings";
const shortString = "TestLots";
const longStringBenchmark = new Benchmarkify("Long string comparison");
const shortStringBenchmark = new Benchmarkify("Short string comparison");
const suiteOptions = { cycles: 100000 };

const suites = [
  {
    name: "snake_case",
    fastCase: toSnakeCase,
    changeCase: snakeCase,
  },
  {
    name: "camelCase",
    fastCase: toCamelCase,
    changeCase: camelCase,
  },
  {
    name: "Sentence case",
    fastCase: toSentenceCase,
    changeCase: sentenceCase,
  },
  {
    name: "Title Case",
    fastCase: toTitleCase,
    changeCase: capitalCase,
  },
  {
    name: "PascalCase",
    fastCase: toPascalCase,
    changeCase: pascalCase,
  },
  {
    name: "kebab-case",
    fastCase: toKebabCase,
    changeCase: paramCase,
  },
  {
    name: "SCREAMING_SNAKE_CASE",
    fastCase: toScreamingSnakeCase,
    changeCase: constantCase,
  },
];

const longStringSuites = suites.map(({ name, fastCase, changeCase }) => {
  const suite = longStringBenchmark.createSuite(name, suiteOptions);
  suite.add("fastCase", function () {
    fastCase(longString);
  });
  suite.add("changeCase", function () {
    changeCase(longString);
  });
  return suite;
});

const shortStringSuites = suites.map(({ name, fastCase, changeCase }) => {
  const suite = shortStringBenchmark.createSuite(name, suiteOptions);
  suite.add("fastCase", function () {
    fastCase(shortString);
  });
  suite.add("changeCase", function () {
    changeCase(shortString);
  });
  return suite;
});

async function main() {
  longStringBenchmark.printHeader();
  await longStringBenchmark.run(longStringSuites);
  shortStringBenchmark.printHeader();
  await shortStringBenchmark.run(shortStringSuites);
}

main();
