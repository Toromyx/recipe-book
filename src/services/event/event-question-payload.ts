import type { EventQuestionChannel } from "./event-question-channel.ts";

type EventQuestionPayloadMap = {
  [EventQuestionChannel.SCRAPER_DOM_CREATE_QUESTION]: string;
  [EventQuestionChannel.SCRAPER_DOM_CANONICAL_LINK_QUESTION]: string;
};

export type EventQuestionPayload<T extends EventQuestionChannel> =
  EventQuestionPayloadMap[T];
