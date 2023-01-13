import { EventChannel } from "./event-channel.js";

type EventAnswerMap = {
  [EventChannel.ENTITY_ACTION_CREATED_INGREDIENT]: void;
  [EventChannel.ENTITY_ACTION_UPDATED_INGREDIENT]: number;
  [EventChannel.ENTITY_ACTION_DELETED_INGREDIENT]: number;
  [EventChannel.ENTITY_ACTION_CREATED_RECIPE]: void;
  [EventChannel.ENTITY_ACTION_UPDATED_RECIPE]: number;
  [EventChannel.ENTITY_ACTION_DELETED_RECIPE]: number;
  [EventChannel.ENTITY_ACTION_CREATED_RECIPE_INGREDIENT]: void;
  [EventChannel.ENTITY_ACTION_UPDATED_RECIPE_INGREDIENT]: number;
  [EventChannel.ENTITY_ACTION_DELETED_RECIPE_INGREDIENT]: number;
  [EventChannel.ENTITY_ACTION_CREATED_RECIPE_STEP]: void;
  [EventChannel.ENTITY_ACTION_UPDATED_RECIPE_STEP]: number;
  [EventChannel.ENTITY_ACTION_DELETED_RECIPE_STEP]: number;
};
export type EventAnswer<T extends EventChannel> = EventAnswerMap[T];
