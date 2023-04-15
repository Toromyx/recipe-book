/*
 * @see https://github.com/tc39/proposal-regex-escaping
 */

if (!RegExp.escape) {
  RegExp.escape = function (string: string) {
    return string.replace(/[\\^$*+?.()|[\]{}]/g, "\\$&");
  };
}

declare global {
  interface RegExpConstructor {
    /**
     * Escape a string for usage in regular expressions.
     * @see https://github.com/tc39/proposal-regex-escaping/blob/main/polyfill.js
     */
    escape(string: string): string;
  }
}
