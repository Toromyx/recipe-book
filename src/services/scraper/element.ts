import { v4 } from "uuid";
import { answer, listen } from "../event/client.js";
import { EventAnswerChannel } from "../event/event-answer-channel.ts";
import { EventChannel } from "../event/event-channel.ts";
import { EventQuestionChannel } from "../event/event-question-channel.ts";

const elements: Record<string, HTMLElement> = {};

export function create(element: HTMLElement): string {
  const id = v4();
  elements[id] = element;
  return id;
}

void answer(
  EventQuestionChannel.SCRAPER_ELEMENT_TEXT_CONTENT_QUESTION,
  EventAnswerChannel.SCRAPER_ELEMENT_TEXT_CONTENT_ANSWER,
  (id) => {
    const element = elements[id];
    return element.textContent ?? "";
  },
);

void answer(
  EventQuestionChannel.SCRAPER_ELEMENT_INNER_TEXT_QUESTION,
  EventAnswerChannel.SCRAPER_ELEMENT_INNER_TEXT_ANSWER,
  (id) => {
    const element = elements[id];
    return element.innerText ?? "";
  },
);

void answer(
  EventQuestionChannel.SCRAPER_ELEMENT_GET_ATTRIBUTE_QUESTION,
  EventAnswerChannel.SCRAPER_ELEMENT_GET_ATTRIBUTE_ANSWER,
  ([id, qualifiedName]) => {
    const element = elements[id];
    return element.getAttribute(qualifiedName) ?? "";
  },
);

void answer(
  EventQuestionChannel.SCRAPER_ELEMENT_SELECT_QUESTION,
  EventAnswerChannel.SCRAPER_ELEMENT_SELECT_ANSWER,
  ([id, selector]) => {
    const parentElement = elements[id];
    const element = parentElement.querySelector<HTMLElement>(selector);
    if (!element) {
      return null;
    }
    return create(element);
  },
);

void answer(
  EventQuestionChannel.SCRAPER_ELEMENT_SELECT_ALL_QUESTION,
  EventAnswerChannel.SCRAPER_ELEMENT_SELECT_ALL_ANSWER,
  ([id, selector]) => {
    const parentElement = elements[id];
    const children = parentElement.querySelectorAll<HTMLElement>(selector);
    return [...children].map((element) => create(element));
  },
);

void listen(EventChannel.SCRAPER_ELEMENT_DROP, (event) => {
  delete elements[event.payload];
});
