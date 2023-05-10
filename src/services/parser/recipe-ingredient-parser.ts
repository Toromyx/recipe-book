/*
 * This module implements simple parsing logic for parsing recipe ingredients from common inputs.
 * In most cases this will be from the user's clipboard.
 */
import { mean, standardDeviation } from "../util/statistics.ts";
import { enNumberParser } from "./number-parser/en-number-parser.ts";
import { userLocaleNumberParser } from "./number-parser/user-locale-number-parser.ts";

/**
 * a single parsed recipe ingredient
 */
export type ParsedRecipeIngredient = {
  quantity?: number;
  unit?: string;
  name: string;
  quality?: string;
};

type AndIndex = {
  index: number;
};

type Extracted = {
  prefix: string;
  suffix: string;
};

const ingredientSeparators = ["\n", ",", ";"];

const groupingSymbols = {
  open: ["(", "{", "[", "<"],
  close: [")", "}", "]", ">"],
};

const getSeparatorButNotInsideGroupingSymbols = (separator: string) =>
  new RegExp(
    `${separator}(?![^${RegExp.escape(
      groupingSymbols.open.join(""),
    )}]*[${RegExp.escape(groupingSymbols.close.join(""))}])`,
  );

/**
 * Parse a provided html string into recipe ingredients.
 *
 * The provided unit list is used to find the ingredient units. It should be a list of known units.
 */
export function parseHtml(
  html: string,
  unitList: string[],
): ParsedRecipeIngredient[] {
  const parser = new DOMParser();
  const document = parser.parseFromString(html, "text/html");
  const recipeIngredients: ParsedRecipeIngredient[] = [];
  // try to parse ingredients from tables in the html
  const tables = document.querySelectorAll("table");
  recipeIngredients.push(
    // for each table ...
    ...[...tables].flatMap(
      (table): ParsedRecipeIngredient[] =>
        // ... get the rows ...
        [...table.rows]
          .map((row): ParsedRecipeIngredient | null =>
            // ... and for each row ...
            fromParts(
              unitList,
              // ... interpret each cell as a part of an ingredient
              ...[...row.cells]
                .map((cell) => cell.innerText.trim())
                .filter(Boolean),
            ),
          )
          .filter(Boolean) as ParsedRecipeIngredient[],
    ),
  );
  // try to parse ingredients from the lists in the html
  const lists = document.querySelectorAll("ol ul");
  recipeIngredients.push(
    // for each list ...
    ...[...lists].flatMap(
      (list): ParsedRecipeIngredient[] =>
        // ... get the list items ...
        [...list.querySelectorAll("li")]
          .map((listItem): ParsedRecipeIngredient | null =>
            fromParts(
              unitList,
              // ... and interpret each part of the item, split by whitespace, as part of an ingredient
              ...listItem.innerText
                .split(/\s+/)
                .map((part) => part.trim())
                .filter(Boolean),
            ),
          )
          .filter(Boolean) as ParsedRecipeIngredient[],
    ),
  );
  // if no ingredients are found in the structured html, fall back to just parsing the text
  if (!recipeIngredients.length) {
    recipeIngredients.push(
      ...parseText(document.documentElement.innerText, unitList),
    );
  }
  return recipeIngredients;
}

/**
 * Parse a provided string into recipe ingredients.
 *
 * The provided unit list is used to find the ingredient units. It should be a list of known units.
 */
export function parseText(
  text: string,
  unitList: string[],
): ParsedRecipeIngredient[] {
  const recipeIngredients: ParsedRecipeIngredient[] = [];
  // First we need to split the text into parts where each part is an ingredient.
  const separators = ingredientSeparators.filter((separator) =>
    text.includes(separator),
  );
  // The separator regular expressions contain a negative lookahead for grouping symbols to not split the text in the middle of grouped text.
  const separatorRegExps = separators.map(
    getSeparatorButNotInsideGroupingSymbols,
  );
  // split the text for each separator
  const splitTextsBySeparator = separatorRegExps.map((separatorRegExp) =>
    text
      .split(separatorRegExp)
      .map((part) => part.trim())
      .filter(Boolean),
  );
  /**
   * Get a ratio between the number of parts in the split text and the mean lengths of those parts.
   *
   * The result will be a number between 0 and 1. A higher number closer to 1 the numbers are close together:
   * If the parts would be written in a list, the resulting block of text would be close to a square.
   */
  const getSquareness = (splitText: string[]): number => {
    const lengths = splitText.map((part) => part.length);
    const lengthsMean = mean(...lengths);
    const lengthAndLengthsMean = [splitText.length, lengthsMean];
    return (
      Math.min(...lengthAndLengthsMean) / Math.max(...lengthAndLengthsMean)
    );
  };
  /**
   * Get the standard deviation of the lengths of the parts of the split text.
   */
  const getLengthsStandardDeviation = (splitText: string[]): number => {
    const lengths = splitText.map((part) => part.length);
    return standardDeviation(...lengths);
  };
  // sort the split texts by some made up score
  splitTextsBySeparator.sort((splitTextA, splitTextB) => {
    const squarenessA = getSquareness(splitTextA);
    const squarenessB = getSquareness(splitTextB);
    // more squareness (higher value) is better
    const squarenessComparison = squarenessB - squarenessA;
    if (squarenessComparison) {
      return squarenessComparison;
    }
    const lengthsStandardDeviationA = getLengthsStandardDeviation(splitTextA);
    const lengthsStandardDeviationB = getLengthsStandardDeviation(splitTextB);
    // lower standard deviation is better
    return lengthsStandardDeviationA - lengthsStandardDeviationB;
  });
  // the best split is the first element
  const splitText = splitTextsBySeparator[0];
  recipeIngredients.push(...parseStrings(splitText, unitList));
  return recipeIngredients;
}

/**
 * Parse the provided strings into recipe ingredients.
 *
 * Each string must be known to be one ingredient.
 * The provided unit list is used to find the ingredient units. It should be a list of known units.
 */
export function parseStrings(
  strings: string[],
  unitList: string[],
): ParsedRecipeIngredient[] {
  return strings
    .map((string) => parseString(string, unitList))
    .filter(Boolean) as ParsedRecipeIngredient[];
}

/**
 * Parse a provided string into a single recipe ingredient.
 *
 * The provided unit list is used to find the ingredient units. It should be a list of known units.
 */
export function parseString(
  string: string,
  unitList: string[],
): ParsedRecipeIngredient | null {
  return fromParts(
    unitList,
    ...string
      .split(getSeparatorButNotInsideGroupingSymbols("\\s"))
      .map((part) => part.trim())
      .filter(Boolean),
  );
}

/**
 * Parse a single recipient ingredient from an arbitrary amount of parts.
 */
function fromParts(
  unitList: string[],
  ...parts: string[]
): ParsedRecipeIngredient | null {
  if (parts.length <= 0) {
    return null;
  }
  // one part: interpret as name
  if (parts.length === 1) {
    return {
      ...extractNameAndQuality(parts[0]),
    };
  }
  const extractedQuantityAndIndex = extractQuantityAndIndex(...parts);
  const extractedUnitAndIndex = extractUnitAndIndex(unitList, ...parts);
  // two parts
  if (parts.length === 2) {
    // if quantity is found, the rest must be the name
    if (extractedQuantityAndIndex) {
      return {
        quantity: extractedQuantityAndIndex.extractedQuantity.quantity,
        unit:
          extractUnitFromExtractedQuantity(
            extractedQuantityAndIndex.extractedQuantity,
          ) || undefined,
        ...extractNameAndQuality(
          parts[(extractedQuantityAndIndex.index + 1) % 2],
        ),
      };
    }
    // if unit is found, the rest must the name
    if (extractedUnitAndIndex) {
      return {
        unit: extractedUnitAndIndex.extractedUnit.unit,
        ...extractNameAndQuality(parts[(extractedUnitAndIndex.index + 1) % 2]),
      };
    }
    // if none found, all must be the name
    return {
      ...extractNameAndQuality(parts.join(" ")),
    };
  }
  // three or more parts: if both quantity and unit are found, use both, otherwise only use the found ones
  if (extractedQuantityAndIndex) {
    let unit = extractUnitFromExtractedQuantity(
      extractedQuantityAndIndex.extractedQuantity,
    );
    if (!unit && extractedUnitAndIndex) {
      unit = extractedUnitAndIndex.extractedUnit.unit;
    }
    let name;
    if (extractedUnitAndIndex) {
      name = parts
        .filter(
          (_, i) =>
            i !== extractedQuantityAndIndex.index &&
            i !== extractedUnitAndIndex.index,
        )
        .join(" ");
    } else {
      name = parts
        .filter((_, i) => i !== extractedQuantityAndIndex.index)
        .join(" ");
    }
    if (unit) {
      return {
        quantity: extractedQuantityAndIndex.extractedQuantity.quantity,
        unit,
        ...extractNameAndQuality(name),
      };
    }
    return {
      quantity: extractedQuantityAndIndex.extractedQuantity.quantity,
      ...extractNameAndQuality(name),
    };
  }
  if (extractedUnitAndIndex) {
    const name = parts
      .filter((_, i) => i !== extractedUnitAndIndex.index)
      .join(" ");
    return {
      unit: extractedUnitAndIndex.extractedUnit.unit,
      ...extractNameAndQuality(name),
    };
  }
  return {
    ...extractNameAndQuality(parts.join(" ")),
  };
}

type NameAndQuality = {
  name: string;
  quality?: string;
};

/**
 * Parse an ingredient name and quality from a single string.
 *
 * The quality is recognized by being enclosed by grouping symbols.
 * When no quality is found, the whole string is returned as the name.
 */
function extractNameAndQuality(name: string): NameAndQuality {
  const qualityRegex = new RegExp(
    `[${RegExp.escape(
      groupingSymbols.open.join(""),
    )}](?<quality>.+)[${RegExp.escape(groupingSymbols.close.join(""))}]`,
    "d",
  );
  const regexpMatchArray = name.match(qualityRegex);
  const quality = regexpMatchArray?.groups?.quality;
  if (quality) {
    name = `${
      // @ts-expect-error indices is defined
      name.slice(0, regexpMatchArray.indices[0][0]).trim()
    } ${
      // @ts-expect-error indices is defined
      name.slice(regexpMatchArray.indices[0][1]).trim()
    }`.trim();
  }
  return {
    name,
    quality,
  };
}

type ExtractedQuantityAndIndex = {
  extractedQuantity: ExtractedQuantity;
} & AndIndex;

/**
 * Find the ingredient quantity in an arbitrary amount of ingredient parts.
 *
 * This method returns the extracted quantity (with prefix and suffix) together with the index of the part where it was found.
 */
function extractQuantityAndIndex(
  ...parts: string[]
): ExtractedQuantityAndIndex | null {
  for (let i = 0; i < parts.length; i++) {
    const part = parts[i];
    const extractedQuantity = extractQuantity(part);
    if (extractedQuantity) {
      return {
        index: i,
        extractedQuantity,
      };
    }
  }
  return null;
}

type ExtractedUnitAndIndex = {
  extractedUnit: ExtractedUnit;
} & AndIndex;

/**
 * Find the ingredient unit in an arbitrary amount of ingredient parts.
 *
 * This method returns the extracted unit together with the index of the part where it was found.
 */
function extractUnitAndIndex(
  unitList: string[],
  ...parts: string[]
): ExtractedUnitAndIndex | null {
  for (let i = 0; i < parts.length; i++) {
    const part = parts[i];
    const extractedUnit = extractUnit(part, unitList);
    if (extractedUnit) {
      return {
        index: i,
        extractedUnit,
      };
    }
  }
  return null;
}

type ExtractedQuantity = {
  quantity: number;
} & Extracted;

/**
 * Try to find an ingredient quantity (number) in an ingredient part.
 *
 * This method also handles fractions (x/y) and ranges (x-y), but not both.
 * The non-quantity parts of the string are returned as prefix and suffix.
 */
function extractQuantity(string: string): ExtractedQuantity | null {
  const userLocaleNumberRegExp = new RegExp(
    userLocaleNumberParser.getRegExpString(),
    "gd",
  );
  let matches = [...string.matchAll(userLocaleNumberRegExp)];
  let numberParser = userLocaleNumberParser;
  if (!matches.length) {
    const enNumberRegExp = new RegExp(enNumberParser.getRegExpString(), "gd");
    matches = [...string.matchAll(enNumberRegExp)];
    numberParser = enNumberParser;
  }
  if (!matches.length) {
    return null;
  }
  let fractionMatches: RegExpMatchArray[] = [];
  let rangeMatches: RegExpMatchArray[] = [];
  // If there is more than one number in the part, try to find a fraction or range.
  if (matches.length > 1) {
    const fractionRegExp = new RegExp(
      `(?<numerator>${numberParser.getRegExpString()})/(?<denominator>${numberParser.getRegExpString()})`,
      "gd",
    );
    fractionMatches = [...string.matchAll(fractionRegExp)];
    const rangeRegExp = new RegExp(
      `(?<start>${numberParser.getRegExpString()})-(?<end>${numberParser.getRegExpString()})`,
      "gd",
    );
    rangeMatches = [...string.matchAll(rangeRegExp)];
  }
  let match = matches[0];
  let quantity = numberParser.parse(match[0]);
  if (fractionMatches.length) {
    match = fractionMatches[0];
    quantity =
      // @ts-expect-error groups is defined
      numberParser.parse(match.groups.numerator) /
      // @ts-expect-error groups is defined
      numberParser.parse(match.groups.denominator);
  }
  if (rangeMatches.length) {
    match = rangeMatches[0];
    quantity =
      // @ts-expect-error groups is defined
      (numberParser.parse(match.groups.end) +
        // @ts-expect-error groups is defined
        numberParser.parse(match.groups.start)) /
      2;
  }
  return {
    // @ts-expect-error indices is defined
    prefix: string.slice(0, match.indices[0][0]).trim(),
    quantity,
    // @ts-expect-error indices is defined
    suffix: string.slice(match.indices[0][1]).trim(),
  };
}

type ExtractedUnit = {
  unit: string;
} & Extracted;

/**
 * Try to find a unit in an ingredient part.
 *
 * The unit must be included in the provided unit list.
 * The non-unit parts of the string are returned as prefix and suffix.
 */
function extractUnit(string: string, unitList: string[]): ExtractedUnit | null {
  for (const unit of unitList) {
    const parts = string.split(
      new RegExp(`(^|\\s+)${RegExp.escape(unit)}($|\\s+)`),
    );
    if (parts.length > 1) {
      return {
        prefix: parts[0].trim(),
        unit,
        suffix: parts.slice(1).join(unit).trim(),
      };
    }
  }

  return null;
}

/**
 * Get the suffix or prefix of the extracted quantity.
 */
function extractUnitFromExtractedQuantity(
  extractedQuantity: ExtractedQuantity,
): string | null {
  if (extractedQuantity.suffix.trim()) {
    return extractedQuantity.suffix.trim();
  }
  if (extractedQuantity.prefix.trim()) {
    return extractedQuantity.prefix.trim();
  }
  return null;
}
