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
