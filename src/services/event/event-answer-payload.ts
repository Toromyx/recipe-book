import type { EventAnswerChannel } from "./event-answer-channel.ts";

type EventAnswerPayloadMap = {
  [EventAnswerChannel.SCRAPER_DOM_CREATE_ANSWER]: string;
  [EventAnswerChannel.SCRAPER_DOM_CANONICAL_LINK_ANSWER]: string;
  [EventAnswerChannel.SCRAPER_DOM_SELECT_ANSWER]: string;
  [EventAnswerChannel.SCRAPER_DOM_SELECT_ALL_ANSWER]: string[];
  [EventAnswerChannel.SCRAPER_ELEMENT_TEXT_CONTENT_ANSWER]: string;
  [EventAnswerChannel.SCRAPER_ELEMENT_GET_ATTRIBUTE_ANSWER]: string;
  [EventAnswerChannel.SCRAPER_ELEMENT_SELECT_ANSWER]: string;
  [EventAnswerChannel.SCRAPER_ELEMENT_SELECT_ALL_ANSWER]: string[];
};

export type EventAnswerPayload<T extends EventAnswerChannel> =
  EventAnswerPayloadMap[T];
