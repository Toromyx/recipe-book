import type { ExternalRecipe } from "../../external-recipe.ts";
import {
  readContent,
  selectInParentNode,
  selectMultipleInParentNode,
} from "../getter.ts";

type PinterestImageSpec = {
  height: number;
  url: string;
  width: number;
};

type PinterestStoryListBlock = {
  heading: string;
  blocks: PinterestStoryItemBlock[];
};

type PinterestStoryItemBlock = {
  text: string;
};

type PinterestStoryPage = {
  blocks: PinterestStoryPageBlock[];
};

type PinterestVideoListItem = {
  thumbnail: string;
  url: string;
};

type PinterestStoryPageBlock = {
  __typename: "StoryPinVideoBlock";
  blockType: 3;
  videoData: {
    videoListEXP3: {
      vEXP3: PinterestVideoListItem;
    };
  };
};

type PinterestRelayResponse = {
  requestParameters: {
    name: string;
  };
  response: {
    data: {
      v3GetPinQuery: {
        data: {
          title: string;
          imageSpec_orig: PinterestImageSpec;
          storyPinData: {
            metadata: {
              pinTitle: string;
              basics: {
                listBlocks: PinterestStoryListBlock[];
              };
            };
            pages: PinterestStoryPage[];
          };
        };
      };
    };
  };
};

function get(data: string): ExternalRecipe {
  const parser = new DOMParser();
  const recipeDocument = parser.parseFromString(data, "text/html");

  const relayResponseElements = recipeDocument.querySelectorAll(
    'script[data-relay-response="true"][type="application/json"]',
  );
  const relayResponses = [...relayResponseElements]
    .map((relayResponseElement) => relayResponseElement?.textContent)
    .filter(Boolean)
    .map(
      (textContent) =>
        JSON.parse(textContent as string) as PinterestRelayResponse,
    );
  const closeupDetailQueryRelayResponse = relayResponses.find(
    (relayResponse) =>
      relayResponse.requestParameters?.name === "CloseupDetailQuery",
  );

  return closeupDetailQueryRelayResponse
    ? externalRecipeFromRelayResponse(closeupDetailQueryRelayResponse)
    : externalRecipeFromDocument(recipeDocument);
}

function externalRecipeFromDocument(recipeDocument: Document): ExternalRecipe {
  return {
    name: readContent(selectInParentNode(recipeDocument, "h1")),
    steps: selectMultipleInParentNode(
      recipeDocument,
      '[data-test-id="collapsible-layout"]',
    ).map((stepElement) => ({
      description: readContent(stepElement),
      ingredients: [],
      files: [],
    })),
  };
}

function externalRecipeFromRelayResponse(
  relayResponse: PinterestRelayResponse,
): ExternalRecipe {
  return {
    name: relayResponse.response.data.v3GetPinQuery.data.title,
    steps: [
      {
        ingredients: [],
        description:
          relayResponse.response.data.v3GetPinQuery.data.storyPinData.metadata.basics.listBlocks
            .map(
              (listBlock) =>
                `${listBlock.heading}\n${listBlock.blocks
                  .map((itemBlock) => itemBlock.text)
                  .join("\n")}`,
            )
            .join("\n"),
        files: relayResponse.response.data.v3GetPinQuery.data.storyPinData.pages
          .flatMap((page) => page.blocks)
          .map((block) => block.videoData.videoListEXP3.vEXP3.url),
      },
    ],
  };
}

/**
 * @type {ExternalRecipeModule}
 */
export { get };
