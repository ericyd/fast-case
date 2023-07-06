// Credit: https://github.com/aarondcohen/benchmark-guid
const Benchmarkify = require("benchmarkify");

const {
  toSnakeCase,
  toCamelCase,
  toSentenceCase,
  toTitleCase,
  toPascalCase,
  toKebabCase,
} = require("../dist/index");

const {
  camelCase,
  capitalCase,
  constantCase,
  dotCase,
  headerCase,
  noCase,
  paramCase,
  pascalCase,
  pathCase,
  sentenceCase,
  snakeCase,
} = require("change-case");

const testString = "SomeWhat varied test-string WITH_MULTIPLE wordCasings";
const benchmark = new Benchmarkify("UUID Processing").printHeader();
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
];

suites.forEach(({ suite, fastCase, changeCase }) => {
  suite.add("fastCase", function () {
    fastCase(testString);
  });
  suite.add("changeCase", function () {
    changeCase(testString);
  });
});

benchmark.run(suites.map(({ suite }) => suite));
