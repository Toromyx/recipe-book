import { IntlMessageFormat } from "intl-messageformat";

type Translations<T> = {
  labels: {
    actions: {
      create: T;
      edit: T;
      update: T;
      delete: T;
      cancel: T;
      confirm: T;
      add: T;
      remove: T;
      file: {
        open: T;
      };
      ocr: T;
    };
    entityFields: {
      ingredient: {
        name: T;
      };
      recipe: {
        name: T;
      };
      recipeFile: {
        name: T;
        path: T;
      };
      recipeIngredient: {
        quantity: T;
        unit: T;
        ingredient: T;
        quality: T;
      };
      recipeStep: {
        description: T;
      };
    };
    descriptions: {
      ocrOutput: T;
    };
    factor: T;
  };
  validity: {
    autocomplete: {
      max: T;
      min: T;
      includesExcluded: T;
    };
    externalRecipeUrlNotSupported: T;
  };
  questions: {
    confirmation: T;
  };
  imperatives: {
    selectPlaceholder: T;
  };
  headings: {
    recipeStep: T;
    ingredients: T;
    description: T;
    files: T;
  };
};

/**
 * This type describes the language-specific messages to be parsed.
 *
 * @see https://unicode-org.github.io/icu/userguide/format_parse/messages/
 */
export type TranslationStrings = Translations<string>;

/**
 * This type describes the created message format objects of the language-specific messages to be formatted.
 */
export type TranslationFormats = Translations<IntlMessageFormat>;

/**
 * Take in message strings and parse them into message format objects.
 *
 * This is implemented via a new proxy object.
 */
export const constructMessageProxy = (
  target: TranslationStrings,
  path = "",
): TranslationFormats =>
  //@ts-expect-error TranslationStrings is mapped to TranslationFormats in this proxy
  new Proxy<TranslationFormats>(target, {
    get(getTarget, getProperty: string | symbol): unknown {
      //@ts-expect-error custom cache field
      if (!this.__cache) {
        //@ts-expect-error custom cache field
        this.__cache = {};
      }
      //@ts-expect-error custom cache field
      // eslint-disable-next-line @typescript-eslint/no-unsafe-member-access
      if (!this.__cache[getProperty]) {
        const newPath = [path, String(getProperty)].filter((e) => e).join(".");
        const targetProperty = getTarget[
          getProperty as keyof TranslationFormats
        ] as unknown;
        switch (typeof targetProperty) {
          case "string":
            //@ts-expect-error custom cache field
            // eslint-disable-next-line @typescript-eslint/no-unsafe-member-access
            this.__cache[getProperty] = new IntlMessageFormat(
              targetProperty,
              "en",
            );
            break;
          case "object":
            //@ts-expect-error custom cache field
            // eslint-disable-next-line @typescript-eslint/no-unsafe-member-access
            this.__cache[getProperty] = constructMessageProxy(
              targetProperty as TranslationStrings,
              newPath,
            );
            break;
          default:
            return { format: () => newPath };
        }
      }
      //@ts-expect-error custom cache field
      // eslint-disable-next-line @typescript-eslint/no-unsafe-member-access
      return this.__cache[getProperty];
    },
  });
