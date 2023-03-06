import { IntlMessageFormat } from "intl-messageformat";

type Translations<T> = {
  labels: {
    actions: {
      create: T;
      edit: T;
      update: T;
      delete: T;
      cancel: T;
      file: {
        open: T;
      };
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
      };
      recipeStep: {
        description: T;
      };
    };
  };
  validity: {
    autocomplete: {
      max: T;
      min: T;
    };
  };
  imperatives: {
    selectPlaceholder: T;
  };
  headings: {
    ingredients: T;
    recipeStep: T;
  };
};

/**
 * @see https://unicode-org.github.io/icu/userguide/format_parse/messages/
 */
export type TranslationStrings = Translations<string>;

export type TranslationFormats = Translations<IntlMessageFormat>;

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
