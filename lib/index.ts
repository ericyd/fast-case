import {
  to_snake_case,
  to_camel_case,
  to_kebab_case,
  to_pascal_case,
  to_sentence_case,
  to_title_case,
  to_screaming_snake_case,
} from "../pkg/index.js";

export const toSnakeCase = (value: string) => to_snake_case(value);
export const toCamelCase = (value: string) => to_camel_case(value);
export const toKebabCase = (value: string) => to_kebab_case(value);
export const toPascalCase = (value: string) => to_pascal_case(value);
export const toSentenceCase = (value: string) => to_sentence_case(value);
export const toTitleCase = (value: string) => to_title_case(value);
export const toScreamingSnakeCase = (value: string) =>
  to_screaming_snake_case(value);
