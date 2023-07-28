import type { Event, UnlistenFn } from "@tauri-apps/api/event";
import { getCurrent } from "@tauri-apps/api/window";
import type { IdPayload } from "../event.ts";
import { debug } from "../log.ts";
import type { EventAnswerChannel } from "./event-answer-channel.ts";
import type { EventAnswerPayload } from "./event-answer-payload.ts";
import type { EventChannel } from "./event-channel.ts";
import type { EventPayload } from "./event-payload.ts";
import type { EventQuestionChannel } from "./event-question-channel.ts";
import type { EventQuestionPayload } from "./event-question-payload.ts";

const currentWindow = getCurrent();

/**
 * Listen to a tauri event.
 */
export function listen<T extends EventChannel>(
  channel: T,
  handler: (event: Event<EventPayload<T>>) => unknown,
): Promise<UnlistenFn> {
  return currentWindow.listen(channel, (event) => {
    debug(`received event on "${channel}"`, event);
    handler(event as Event<EventPayload<T>>);
  });
}

/**
 * Listen to a tauri event and answer on another channel.
 */
export function answer<
  QuestionChannel extends EventQuestionChannel,
  AnswerChannel extends EventAnswerChannel,
>(
  questionChannel: QuestionChannel,
  answerChannel: AnswerChannel,
  handler: (
    payload: EventQuestionPayload<QuestionChannel>,
  ) => EventAnswerPayload<AnswerChannel>,
): Promise<UnlistenFn> {
  return currentWindow.listen(
    questionChannel,
    (event: Event<IdPayload<EventQuestionPayload<QuestionChannel>>>) => {
      debug(`received question on "${questionChannel}"`, event);
      const payloadId = event.payload.id;
      const answer = handler(event.payload.data);
      const answerIdPayload: IdPayload<EventAnswerPayload<AnswerChannel>> = {
        id: payloadId,
        data: answer,
      };
      debug(`answering on "${answerChannel}"`, answer);
      currentWindow.emit(answerChannel, answerIdPayload).catch((reason) => {
        throw reason;
      });
    },
  );
}
