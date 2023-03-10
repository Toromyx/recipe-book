import "../polyfill/regex-escaping.ts";

export class NumberParser {
  #groupSeparator: string;

  #groupSeparatorRegExpEscaped: string;

  #decimalSeparator: string;

  #decimalSeparatorRegExpEscaped: string;

  constructor(locale?: string) {
    const parts = new Intl.NumberFormat(locale).formatToParts(1234567.89);
    this.#groupSeparator =
      parts.find((part) => part.type === "group")?.value ?? ",";
    this.#groupSeparatorRegExpEscaped = RegExp.escape(this.#groupSeparator);
    this.#decimalSeparator =
      parts.find((part) => part.type === "decimal")?.value ?? ".";
    this.#decimalSeparatorRegExpEscaped = RegExp.escape(this.#decimalSeparator);
  }

  parse(string: string): number {
    return Number(
      string
        .trim()
        .replaceAll(this.#groupSeparator, "")
        .replace(this.#decimalSeparator, "."),
    );
  }

  match(string: string): IterableIterator<RegExpMatchArray> {
    return string.matchAll(
      new RegExp(
        `(\\d[\\d${this.#groupSeparatorRegExpEscaped}]*${
          this.#decimalSeparatorRegExpEscaped
        }\\d+|\\d[\\d${this.#groupSeparatorRegExpEscaped}]*|${
          this.#decimalSeparatorRegExpEscaped
        }[\\d]+)`,
        "gd",
      ),
    );
  }
}
