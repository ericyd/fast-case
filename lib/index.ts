import { convert_case } from "../pkg/index.js";

type Case =
  | "camelCase"
  | "snake_case"
  | "Sentence case"
  | "Title Case"
  | "PascalCase"
  | "kebab-case";

export function convertCase(newCase: Case, value: string): string {
  return convert_case(newCase, value);
}

export const toSnakeCase = (value: string) => convertCase("snake_case", value);

export const toCamelCase = (value: string) => convertCase("camelCase", value);

export const toSentenceCase = (value: string) =>
  convertCase("Sentence case", value);

export const toTitleCase = (value: string) => convertCase("Title Case", value);

export const toPascalCase = (value: string) => convertCase("PascalCase", value);

export const toKebabCase = (value: string) => convertCase("kebab-case", value);
