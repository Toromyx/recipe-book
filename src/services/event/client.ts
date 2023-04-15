import type { Event, UnlistenFn } from "@tauri-apps/api/event";
import { getCurrent } from "@tauri-apps/api/window";
import { debug } from "../log.ts";
import type { EventAnswer } from "./event-answer.ts";
import type { EventChannel } from "./event-channel.ts";

const currentWindow = getCurrent();

/**
 * Listen to a tauri event.
 */
export function listen<T extends EventChannel>(
  channel: T,
  handler: (event: Event<EventAnswer<T>>) => unknown,
): Promise<UnlistenFn> {
  return currentWindow.listen(channel, (event) => {
    debug(`received event on "${channel}"`, event);
    handler(event as Event<EventAnswer<T>>);
  });
}
