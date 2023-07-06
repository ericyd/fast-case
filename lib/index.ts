import { convert_case } from "../pkg/index.js";

type Case = "camelCase" | "snake_case";
export function convertCase(newCase: Case, value: string): string {
  return convert_case(newCase, value);
}

export const toSnakeCase = (value: string) => convertCase("snake_case", value);
export const toCamelCase = (value: string) => convertCase("camelCase", value);
