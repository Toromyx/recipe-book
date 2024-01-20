import configJs from "@eslint/js";
import pluginTypescript from "@typescript-eslint/eslint-plugin";
import parserTypescript from "@typescript-eslint/parser";
import configPrettier from "eslint-config-prettier";
import pluginImport from "eslint-plugin-import";
import pluginPromise from "eslint-plugin-promise";
import pluginSvelte from "eslint-plugin-svelte";
import globals from "globals";
import parserSvelte from "svelte-eslint-parser";

/** @type {FlatConfig} */
const baseConfig = {
  plugins: {
    import: pluginImport,
    promise: pluginPromise,
  },
  languageOptions: {
    ecmaVersion: "latest",
    sourceType: "module",
    globals: {
      ...globals.browser,
      ...globals.node,
    },
  },
  rules: {
    ...configJs.configs.recommended.rules,
    ...pluginPromise.configs.recommended.rules,
    ...configPrettier.rules,
    eqeqeq: "warn",
    "guard-for-in": "warn",
    "no-console": "warn",
    "prefer-arrow-callback": "warn",

    "import/order": [
      "warn",
      {
        alphabetize: {
          order: "asc",
        },
      },
    ],
    "import/no-duplicates": "warn",

    "promise/always-return": [
      "warn",
      {
        ignoreLastCallback: true,
      },
    ],
  },
};

/** @type {FlatConfig[]} */
const config = [
  {
    ...baseConfig,
    files: ["**/*.js"],
  },
  {
    ...baseConfig,
    files: ["**/*.ts"],
    plugins: {
      ...baseConfig.plugins,
      "@typescript-eslint": pluginTypescript,
    },
    languageOptions: {
      ...baseConfig.languageOptions,
      parser: parserTypescript,
      parserOptions: {
        project: "./tsconfig.json",
      },
    },
    settings: {
      ...pluginImport.configs.typescript.settings,
      "import/resolver": {
        typescript: true,
        node: true,
      },
    },
    rules: {
      ...baseConfig.rules,
      ...pluginTypescript.configs["eslint-recommended"].rules,
      ...pluginTypescript.configs.recommended.rules,
      ...pluginTypescript.configs["recommended-requiring-type-checking"].rules,
      ...pluginImport.configs.typescript.rules,
      ...configPrettier.rules,
      "@typescript-eslint/consistent-type-imports": "warn",
    },
  },
  {
    ...baseConfig,
    files: ["**/*.d.ts"],
  },
  {
    ...baseConfig,
    files: ["**/*.svelte"],
    plugins: {
      ...baseConfig.plugins,
      svelte: pluginSvelte,
    },
    languageOptions: {
      ...baseConfig.languageOptions,
      parser: parserSvelte,
    },
    rules: {
      ...baseConfig.rules,
      ...pluginSvelte.configs.base.overrides[0].rules,
      ...pluginSvelte.configs.recommended.rules,
      ...pluginSvelte.configs.prettier.rules,
      ...configPrettier.rules,
    },
  },
  {
    ignores: ["dist/**", "src-tauri/target/**"],
  },
];
export default config;
