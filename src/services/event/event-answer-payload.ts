import type { EventAnswerChannel } from "./event-answer-channel.ts";

type EventAnswerPayloadMap = {
  [EventAnswerChannel.SCRAPER_DOM_CREATE_ANSWER]: string;
  [EventAnswerChannel.SCRAPER_DOM_CANONICAL_LINK_ANSWER]: string;
};

export type EventAnswerPayload<T extends EventAnswerChannel> =
  EventAnswerPayloadMap[T];
