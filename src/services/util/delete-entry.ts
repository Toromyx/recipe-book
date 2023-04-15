/**
 * Delete an item/entry in an array/object.
 */
export function deleteEntry<T>(
  objectOrArray: object | Array<T>,
  key: keyof typeof objectOrArray,
): void {
  if (Array.isArray(objectOrArray)) {
    objectOrArray.splice(key, 1);
  }

  delete objectOrArray[key];
}
