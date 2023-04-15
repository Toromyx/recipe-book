/**
 * Set the validity of an HTML input element.
 *
 * Each validator function of the array is called without any parameters.
 * If the validator function returns a truthy message, it is joined together with other messages as the validity of the input.
 */
export function setCustomValidity(
  input: HTMLInputElement,
  ...validators: Array<() => string | undefined>
): void {
  const validityMessages = [];
  for (const validator of validators) {
    const validityMessage = validator();
    if (validityMessage) {
      validityMessages.push(validityMessage);
    }
  }
  input.setCustomValidity(validityMessages.join(" "));
}
