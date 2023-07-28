import { v4 } from "uuid";
import { answer, listen } from "./event/client.ts";
import { EventAnswerChannel } from "./event/event-answer-channel.ts";
import { EventChannel } from "./event/event-channel.ts";
import { EventQuestionChannel } from "./event/event-question-channel.ts";
import { error } from "./log.ts";

const documents: Record<string, Document> = {};

void answer(
  EventQuestionChannel.SCRAPER_DOM_CREATE_QUESTION,
  EventAnswerChannel.SCRAPER_DOM_CREATE_ANSWER,
  (html) => {
    const parser = new DOMParser();
    const document = parser.parseFromString(html, "text/html");
    const id = v4();
    documents[id] = document;
    return id;
  },
);

void answer(
  EventQuestionChannel.SCRAPER_DOM_CANONICAL_LINK_QUESTION,
  EventAnswerChannel.SCRAPER_DOM_CANONICAL_LINK_ANSWER,
  (id) => {
    try {
      const document = documents[id];
      const canonicalLinkElement = document.querySelector(
        'link[rel="canonical"]',
      );
      if (!canonicalLinkElement) {
        return "";
      }
      return canonicalLinkElement.getAttribute("href") ?? "";
    } catch (e) {
      error(e);
      return "";
    }
  },
);

void listen(EventChannel.SCRAPER_DOM_DROP, (event) => {
  delete documents[event.payload];
});
