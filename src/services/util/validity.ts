/**
 * Set the validity of an HTML input element.
 *
 * Each validator function of the array is called without any parameters.
 * If the validator function returns a truthy message, it is joined together with other messages as the validity of the input.
 */
export function setCustomValidity<T>(
  input: HTMLInputElement | undefined,
  value: T,
  ...validators: Array<(value: T) => string | undefined>
): void {
  const validityMessages = [];
  for (const validator of validators) {
    const validityMessage = validator(value);
    if (validityMessage) {
      validityMessages.push(validityMessage);
    }
  }
  input?.setCustomValidity(validityMessages.join(" "));
}
