/**
 * This module implements simple parsing logic for parsing recipe ingredients from common inputs.
 * In most cases this will be from the user's clipboard.
 */
import { mean, standardDeviation } from "../util/statistics.ts";
import { enNumberParser } from "./number-parser/en-number-parser.ts";
import { userLocaleNumberParser } from "./number-parser/user-locale-number-parser.ts";

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

const contextDelimiters = {
  open: ["(", "{", "[", "<"],
  close: [")", "}", "]", ">"],
};

export function parseHtml(
  html: string,
  unitList: string[],
): ParsedRecipeIngredient[] {
  const parser = new DOMParser();
  const document = parser.parseFromString(html, "text/html");
  const recipeIngredients: ParsedRecipeIngredient[] = [];
  const tables = document.querySelectorAll("table");
  recipeIngredients.push(
    ...[...tables].flatMap((table): ParsedRecipeIngredient[] => {
      return [...table.rows]
        .map((row): ParsedRecipeIngredient | null => {
          return fromParts(
            unitList,
            ...[...row.cells]
              .map((cell) => cell.innerText.trim())
              .filter(Boolean),
          );
        })
        .filter(Boolean) as ParsedRecipeIngredient[];
    }),
  );
  const lists = document.querySelectorAll("ol ul");
  recipeIngredients.push(
    ...[...lists].flatMap((list): ParsedRecipeIngredient[] => {
      return [...list.querySelectorAll("li")]
        .map((listItem): ParsedRecipeIngredient | null => {
          return fromParts(
            unitList,
            ...listItem.innerText.split(/\s+/).filter(Boolean),
          );
        })
        .filter(Boolean) as ParsedRecipeIngredient[];
    }),
  );
  if (!recipeIngredients.length) {
    recipeIngredients.push(
      ...parseText(document.documentElement.innerText, unitList),
    );
  }
  return recipeIngredients;
}

export function parseText(
  text: string,
  unitList: string[],
): ParsedRecipeIngredient[] {
  const recipeIngredients: ParsedRecipeIngredient[] = [];
  const separators = ingredientSeparators.filter((separator) =>
    text.includes(separator),
  );
  const separatorRegExps = separators.map(
    (separator) =>
      new RegExp(
        `${separator}(?![^${RegExp.escape(
          contextDelimiters.open.join(""),
        )}]*[${RegExp.escape(contextDelimiters.close.join(""))}])`,
      ),
  );
  const splitTextsBySeparator = separatorRegExps.map((separatorRegExp) =>
    text
      .split(separatorRegExp)
      .map((part) => part.trim())
      .filter(Boolean),
  );
  const getSquareness = (splitText: string[]): number => {
    const lengths = splitText.map((part) => part.length);
    const lengthsMean = mean(...lengths);
    const lengthAndLengthsMean = [splitText.length, lengthsMean];
    return (
      Math.min(...lengthAndLengthsMean) / Math.max(...lengthAndLengthsMean)
    );
  };
  const getLengthsStandardDeviation = (splitText: string[]): number => {
    const lengths = splitText.map((part) => part.length);
    return standardDeviation(...lengths);
  };
  splitTextsBySeparator.sort((splitTextA, splitTextB) => {
    const squarenessA = getSquareness(splitTextA);
    const squarenessB = getSquareness(splitTextB);
    // higher is better
    const squarenessComparison = squarenessB - squarenessA;
    if (squarenessComparison) {
      return squarenessComparison;
    }
    const lengthsStandardDeviationA = getLengthsStandardDeviation(splitTextA);
    const lengthsStandardDeviationB = getLengthsStandardDeviation(splitTextB);
    // lower is better
    return lengthsStandardDeviationA - lengthsStandardDeviationB;
  });
  const splitText = splitTextsBySeparator[0];
  recipeIngredients.push(
    ...(splitText
      .map((line) => fromParts(unitList, ...line.split(/\s+/).filter(Boolean)))
      .filter(Boolean) as ParsedRecipeIngredient[]),
  );
  return recipeIngredients;
}

function fromParts(
  unitList: string[],
  ...parts: string[]
): ParsedRecipeIngredient | null {
  if (parts.length <= 0) {
    return null;
  }
  if (parts.length === 1) {
    return {
      ...extractNameAndQuality(parts[0]),
    };
  }
  const extractedQuantityAndIndex = extractQuantityAndIndex(...parts);
  const extractedUnitAndIndex = extractUnitAndIndex(unitList, ...parts);
  if (parts.length === 2) {
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
    if (extractedUnitAndIndex) {
      return {
        unit: extractedUnitAndIndex.extractedUnit.unit,
        ...extractNameAndQuality(parts[(extractedUnitAndIndex.index + 1) % 2]),
      };
    }
    return {
      ...extractNameAndQuality(parts.join(" ")),
    };
  }
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

function extractNameAndQuality(name: string): NameAndQuality {
  const qualityRegex = new RegExp(
    `[${RegExp.escape(
      contextDelimiters.open.join(""),
    )}](?<quality>.+)[${RegExp.escape(contextDelimiters.close.join(""))}]`,
    "d",
  );
  const regexpMatchArray = name.match(qualityRegex);
  const quality = regexpMatchArray?.groups?.quality;
  if (quality) {
    name =
      // @ts-expect-error indices is defined
      name.slice(0, regexpMatchArray.indices[0][0]) +
      // @ts-expect-error indices is defined
      name.slice(regexpMatchArray.indices[0][1]);
  }
  return {
    name,
    quality,
  };
}

type ExtractedQuantityAndIndex = {
  extractedQuantity: ExtractedQuantity;
} & AndIndex;

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
    prefix: string.slice(0, match.indices[0][0]),
    quantity,
    // @ts-expect-error indices is defined
    suffix: string.slice(match.indices[0][1]),
  };
}

type ExtractedUnit = {
  unit: string;
} & Extracted;

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
