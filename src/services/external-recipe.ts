export type ExternalRecipeData = {
  data: string;
  instructions: Instructions;
};

export type Instructions = { jsModule: { name: string } };

export type ExternalRecipe = {
  name: string;
  steps: Array<{
    ingredients: string[];
    description: string;
    files: string[];
  }>;
};
