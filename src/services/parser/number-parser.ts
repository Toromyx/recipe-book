/**
 * This class implements localized number parsing functionality by hijacking localized number formatting.
 *
 * It only handles arabic numbers and differences in the type and position of the group and decimal separators.
 */
export class NumberParser {
  #groupSeparator: string;

  #decimalSeparator: string;

  #regExpString: string;

  /**
   * Construct a number parse for a specific locale or the users default locale.
   */
  constructor(locale?: string) {
    // get the parts of a formatted decimal number long enough to have a group separator.
    const parts = new Intl.NumberFormat(locale).formatToParts(1234567.89);
    // get the group separator
    this.#groupSeparator =
      parts.find((part) => part.type === "group")?.value ?? ",";
    const groupSeparatorRegExpEscaped = RegExp.escape(this.#groupSeparator);
    // get the decimal separator
    this.#decimalSeparator =
      parts.find((part) => part.type === "decimal")?.value ?? ".";
    const decimalSeparatorRegExpEscaped = RegExp.escape(this.#decimalSeparator);
    // define a regular expression string which captures a number
    this.#regExpString = `(\\d[\\d${groupSeparatorRegExpEscaped}]*${decimalSeparatorRegExpEscaped}\\d+|\\d[\\d${groupSeparatorRegExpEscaped}]*|${decimalSeparatorRegExpEscaped}[\\d]+)`;
  }

  /**
   * Parse a string to a number by removing all group separators and replacing decimal separators by a dot.
   */
  parse(string: string): number {
    return Number(
      string
        .trim()
        .replaceAll(this.#groupSeparator, "")
        .replace(this.#decimalSeparator, "."),
    );
  }

  /**
   * Get the regular expression string which can capture a number inside a larger string.
   */
  getRegExpString(): string {
    return this.#regExpString;
  }
}
