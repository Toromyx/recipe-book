import { v4 } from "uuid";
import { answer, listen } from "../event/client.ts";
import { EventAnswerChannel } from "../event/event-answer-channel.ts";
import { EventChannel } from "../event/event-channel.ts";
import { EventQuestionChannel } from "../event/event-question-channel.ts";
import { create as createElement } from "./element.ts";

const documents: Record<string, Document> = {};

function create(html: string): string {
  const parser = new DOMParser();
  const document = parser.parseFromString(html, "text/html");
  const id = v4();
  documents[id] = document;
  return id;
}

void answer(
  EventQuestionChannel.SCRAPER_DOM_CREATE_QUESTION,
  EventAnswerChannel.SCRAPER_DOM_CREATE_ANSWER,
  (html) => create(html),
);

void answer(
  EventQuestionChannel.SCRAPER_DOM_SELECT_QUESTION,
  EventAnswerChannel.SCRAPER_DOM_SELECT_ANSWER,
  ([id, selector]) => {
    const document = documents[id];
    const element = document.querySelector<HTMLElement>(selector);
    if (!element) {
      return null;
    }
    return createElement(element);
  },
);

void answer(
  EventQuestionChannel.SCRAPER_DOM_SELECT_ALL_QUESTION,
  EventAnswerChannel.SCRAPER_DOM_SELECT_ALL_ANSWER,
  ([id, selector]) => {
    const document = documents[id];
    const elements = document.querySelectorAll<HTMLElement>(selector);
    return [...elements].map((element) => createElement(element));
  },
);

void listen(EventChannel.SCRAPER_DOM_DROP, (event) => {
  delete documents[event.payload];
});
