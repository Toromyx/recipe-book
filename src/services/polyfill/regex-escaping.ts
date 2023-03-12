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
     * This is a polyfill providing a method for escaping strings for usage in regular expressions.
     * @see https://github.com/tc39/proposal-regex-escaping
     */
    escape(string: string): string;
  }
}
