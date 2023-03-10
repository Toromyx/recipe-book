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
    escape(string: string): string;
  }
}
