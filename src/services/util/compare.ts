/**
 * @return true if elements in the arrays are the same (strict comparison), including their order
 */
export function equalArray(arr1: unknown[], arr2: unknown[]): boolean {
  return (
    arr1.length === arr2.length &&
    arr1.every((value, index) => value === arr2[index])
  );
}

/**
 * @return true if all properties in the objects are the same (strict and deep comparison)
 */
export function equalObject(
  obj1: undefined | object | boolean | number | string,
  obj2: undefined | object | boolean | number | string,
): boolean {
  if (obj1 === obj2) {
    return true;
  }
  if (typeof obj1 !== typeof obj2) {
    return false;
  }
  if (typeof obj1 !== "object" || typeof obj2 !== "object") {
    return false;
  }
  if (Object.keys(obj1).length !== Object.keys(obj2).length) {
    return false;
  }
  return Object.keys(obj1).every((key) =>
    equalObject(obj1[key as keyof object], obj2[key as keyof object]),
  );
}
