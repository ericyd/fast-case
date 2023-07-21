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
const benchmark = new Benchmarkify("Case change comparison").printHeader();
const suiteOptions = { cycles: 100000 };

const suites = [
  {
    suite: benchmark.createSuite("snake_case", suiteOptions),
    fastCase: toSnakeCase,
    changeCase: snakeCase,
  },
  {
    suite: benchmark.createSuite("camelCase", suiteOptions),
    fastCase: toCamelCase,
    changeCase: camelCase,
  },
  {
    suite: benchmark.createSuite("Sentence case", suiteOptions),
    fastCase: toSentenceCase,
    changeCase: sentenceCase,
  },
  {
    suite: benchmark.createSuite("Title Case", suiteOptions),
    fastCase: toTitleCase,
    changeCase: capitalCase,
  },
  {
    suite: benchmark.createSuite("PascalCase", suiteOptions),
    fastCase: toPascalCase,
    changeCase: pascalCase,
  },
  {
    suite: benchmark.createSuite("kebab-case", suiteOptions),
    fastCase: toKebabCase,
    changeCase: paramCase,
  },
  {
    suite: benchmark.createSuite("SCREAMING_SNAKE_CASE", suiteOptions),
    fastCase: toScreamingSnakeCase,
    changeCase: constantCase,
  },
];

suites.forEach(({ suite, fastCase, changeCase }) => {
  suite.add("fastCase", function () {
    fastCase(longString);
    fastCase(shortString);
  });
  suite.add("changeCase", function () {
    changeCase(longString);
    changeCase(shortString);
  });
});

benchmark.run(suites.map(({ suite }) => suite));
