/**
 * This module implements simple parsing logic for parsing recipe ingredients from common inputs.
 * In most cases this will be from the user's clipboard.
 */
import { enNumberParser } from "./number-parser/en-number-parser.ts";
import { userLocaleNumberParser } from "./number-parser/user-locale-number-parser.ts";

export type ParsedRecipeIngredient = {
  quantity?: number;
  unit?: string;
  name: string;
};

export function parseHtml(html: string): ParsedRecipeIngredient[] {
  const parser = new DOMParser();
  const document = parser.parseFromString(html, "text/html");
  const recipeIngredients: ParsedRecipeIngredient[] = [];
  const tables = document.querySelectorAll("table");
  recipeIngredients.push(
    ...[...tables].flatMap((table): ParsedRecipeIngredient[] => {
      return [...table.rows]
        .map((row): ParsedRecipeIngredient | null => {
          return fromParts(
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
          return fromParts(...listItem.innerText.split(/\s+/).filter(Boolean));
        })
        .filter(Boolean) as ParsedRecipeIngredient[];
    }),
  );
  if (!recipeIngredients.length) {
    recipeIngredients.push(...parseText(document.documentElement.innerText));
  }
  return recipeIngredients;
}

export function parseText(text: string): ParsedRecipeIngredient[] {
  const recipeIngredients: ParsedRecipeIngredient[] = [];
  recipeIngredients.push(
    ...(text
      .split("\n")
      .filter((e) => e.trim())
      .map((line) => fromParts(...line.split(/\s+/).filter(Boolean)))
      .filter(Boolean) as ParsedRecipeIngredient[]),
  );
  return recipeIngredients;
}

function fromParts(...parts: string[]): ParsedRecipeIngredient | null {
  if (parts.length <= 0) {
    return null;
  }
  if (parts.length === 1) {
    return {
      name: parts[0],
    };
  }
  if (parts.length === 2) {
    const extractedQuantityAndIndex = extractQuantityAndIndex(...parts);
    if (!extractedQuantityAndIndex) {
      return {
        name: parts.join(" "),
      };
    }
    return {
      quantity: extractedQuantityAndIndex.extractedQuantity.quantity,
      unit:
        extractUnit(extractedQuantityAndIndex.extractedQuantity) || undefined,
      name: parts[extractedQuantityAndIndex.index + (1 % 2)],
    };
  }
  const extractedQuantityAndIndex = extractQuantityAndIndex(...parts);
  if (!extractedQuantityAndIndex) {
    return {
      name: parts.join(" "),
    };
  }
  const unit = extractUnit(extractedQuantityAndIndex.extractedQuantity);
  if (extractedQuantityAndIndex.index === 0) {
    if (unit) {
      return {
        quantity: extractedQuantityAndIndex.extractedQuantity.quantity,
        unit,
        name: parts.slice(1).join(" "),
      };
    }
    return {
      quantity: extractedQuantityAndIndex.extractedQuantity.quantity,
      unit: parts[1],
      name: parts.slice(2).join(" "),
    };
  }
  if (extractedQuantityAndIndex.index === parts.length - 1) {
    if (unit) {
      return {
        quantity: extractedQuantityAndIndex.extractedQuantity.quantity,
        unit,
        name: parts.slice(0, -1).join(" "),
      };
    }
    return {
      quantity: extractedQuantityAndIndex.extractedQuantity.quantity,
      unit: parts[parts.length - 2],
      name: parts.slice(0, -2).join(" "),
    };
  }
  if (unit) {
    return {
      quantity: extractedQuantityAndIndex.extractedQuantity.quantity,
      unit,
      name: parts
        .filter((_, i) => i !== extractedQuantityAndIndex.index)
        .join(" "),
    };
  }
  const partsBefore = parts.slice(0, extractedQuantityAndIndex.index);
  const partsAfter = parts.slice(extractedQuantityAndIndex.index + 1);
  const unitPartsBefore = partsBefore < partsAfter;
  if (unitPartsBefore) {
    return {
      quantity: extractedQuantityAndIndex.extractedQuantity.quantity,
      unit: partsBefore.join(" "),
      name: partsAfter.join(" "),
    };
  }
  return {
    quantity: extractedQuantityAndIndex.extractedQuantity.quantity,
    unit: partsAfter.join(" "),
    name: partsBefore.join(" "),
  };
}

type ExtractedQuantityAndIndex = {
  index: number;
  extractedQuantity: ExtractedQuantity;
};

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

type ExtractedQuantity = {
  prefix: string;
  quantity: number;
  suffix: string;
};

function extractQuantity(string: string): ExtractedQuantity | null {
  let matches = [...userLocaleNumberParser.match(string)];
  let numberParser = userLocaleNumberParser;
  if (!matches.length) {
    matches = [...enNumberParser.match(string)];
    numberParser = enNumberParser;
  }
  if (!matches.length) {
    return null;
  }
  const match = matches[0];
  return {
    // @ts-expect-error indices is defined
    prefix: string.slice(0, match.indices[0][0]),
    quantity: numberParser.parse(match[0]),
    // @ts-expect-error indices is defined
    suffix: string.slice(match.indices[0][1]),
  };
}

function extractUnit(extractedQuantity: ExtractedQuantity): string | null {
  if (extractedQuantity.suffix.trim()) {
    return extractedQuantity.suffix.trim();
  }
  if (extractedQuantity.prefix.trim()) {
    return extractedQuantity.prefix.trim();
  }
  return null;
}
