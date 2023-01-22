import type { Event, UnlistenFn } from "@tauri-apps/api/event";
import { getCurrent } from "@tauri-apps/api/window";
import { debug } from "../log.js";
import type { EventChannel } from "./event-channel.js";
import type { EventAnswer } from "./event-answer.js";

type Client = {
  listen<T extends EventChannel>(
    channel: T,
    handler: (event: Event<EventAnswer<T>>) => void,
  ): Promise<UnlistenFn>;
};

const currentWindow = getCurrent();

export const client: Client = {
  listen<T extends EventChannel>(
    channel: T,
    handler: (event: Event<EventAnswer<T>>) => void,
  ): Promise<UnlistenFn> {
    return currentWindow.listen(channel, (event) => {
      debug(`received event on "${channel}"`, event);
      handler(event as Event<EventAnswer<T>>);
    });
  },
};
