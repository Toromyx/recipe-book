import type { EventChannel } from "./event-channel.ts";

type EventPayloadMap = {
  [EventChannel.ENTITY_ACTION_CREATED_INGREDIENT]: void;
  [EventChannel.ENTITY_ACTION_UPDATED_INGREDIENT]: number;
  [EventChannel.ENTITY_ACTION_DELETED_INGREDIENT]: number;
  [EventChannel.ENTITY_ACTION_CREATED_RECIPE]: void;
  [EventChannel.ENTITY_ACTION_UPDATED_RECIPE]: number;
  [EventChannel.ENTITY_ACTION_DELETED_RECIPE]: number;
  [EventChannel.ENTITY_ACTION_CREATED_RECIPE_FILE]: void;
  [EventChannel.ENTITY_ACTION_UPDATED_RECIPE_FILE]: number;
  [EventChannel.ENTITY_ACTION_DELETED_RECIPE_FILE]: number;
  [EventChannel.ENTITY_ACTION_CREATED_RECIPE_INGREDIENT]: void;
  [EventChannel.ENTITY_ACTION_UPDATED_RECIPE_INGREDIENT]: number;
  [EventChannel.ENTITY_ACTION_DELETED_RECIPE_INGREDIENT]: number;
  [EventChannel.ENTITY_ACTION_CREATED_RECIPE_INGREDIENT_DRAFT]: void;
  [EventChannel.ENTITY_ACTION_UPDATED_RECIPE_INGREDIENT_DRAFT]: number;
  [EventChannel.ENTITY_ACTION_DELETED_RECIPE_INGREDIENT_DRAFT]: number;
  [EventChannel.ENTITY_ACTION_CREATED_RECIPE_STEP]: void;
  [EventChannel.ENTITY_ACTION_UPDATED_RECIPE_STEP]: number;
  [EventChannel.ENTITY_ACTION_DELETED_RECIPE_STEP]: number;
  [EventChannel.ENTITY_ACTION_CREATED_UNIT_NAME]: void;
  [EventChannel.ENTITY_ACTION_UPDATED_UNIT_NAME]: number;
  [EventChannel.ENTITY_ACTION_DELETED_UNIT_NAME]: number;

  [EventChannel.SCRAPER_DOM_DROP]: string;
};

export type EventPayload<T extends EventChannel> = EventPayloadMap[T];