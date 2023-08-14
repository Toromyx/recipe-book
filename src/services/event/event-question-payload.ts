import type { EventQuestionChannel } from "./event-question-channel.ts";

type EventQuestionPayloadMap = {
  [EventQuestionChannel.SCRAPER_DOM_CREATE_QUESTION]: string;
  [EventQuestionChannel.SCRAPER_DOM_CANONICAL_LINK_QUESTION]: string;
  [EventQuestionChannel.SCRAPER_DOM_SELECT_QUESTION]: [string, string];
  [EventQuestionChannel.SCRAPER_DOM_SELECT_ALL_QUESTION]: [string, string];
  [EventQuestionChannel.SCRAPER_ELEMENT_TEXT_CONTENT_QUESTION]: string;
  [EventQuestionChannel.SCRAPER_ELEMENT_GET_ATTRIBUTE_QUESTION]: [
    string,
    string,
  ];
  [EventQuestionChannel.SCRAPER_ELEMENT_SELECT_QUESTION]: [string, string];
  [EventQuestionChannel.SCRAPER_ELEMENT_SELECT_ALL_QUESTION]: [string, string];
};

export type EventQuestionPayload<T extends EventQuestionChannel> =
  EventQuestionPayloadMap[T];
