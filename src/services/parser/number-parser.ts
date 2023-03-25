export class NumberParser {
  #groupSeparator: string;

  #decimalSeparator: string;

  #regExpString: string;

  constructor(locale?: string) {
    const parts = new Intl.NumberFormat(locale).formatToParts(1234567.89);
    this.#groupSeparator =
      parts.find((part) => part.type === "group")?.value ?? ",";
    const groupSeparatorRegExpEscaped = RegExp.escape(this.#groupSeparator);
    this.#decimalSeparator =
      parts.find((part) => part.type === "decimal")?.value ?? ".";
    const decimalSeparatorRegExpEscaped = RegExp.escape(this.#decimalSeparator);
    this.#regExpString = `(\\d[\\d${groupSeparatorRegExpEscaped}]*${decimalSeparatorRegExpEscaped}\\d+|\\d[\\d${groupSeparatorRegExpEscaped}]*|${decimalSeparatorRegExpEscaped}[\\d]+)`;
  }

  parse(string: string): number {
    return Number(
      string
        .trim()
        .replaceAll(this.#groupSeparator, "")
        .replace(this.#decimalSeparator, "."),
    );
  }

  getRegExpString(): string {
    return this.#regExpString;
  }
}
