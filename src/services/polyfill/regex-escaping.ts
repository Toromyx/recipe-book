/*
 * @see https://github.com/tc39/proposal-regex-escaping
 */

// @ts-expect-error This is a polyfill on the RegExp constructor.
if (!RegExp.escape) {
  // @ts-expect-error see above
  RegExp.escape = function (string: string) {
    return string.replace(/[\\^$*+?.()|[\]{}]/g, "\\$&");
  };
}
