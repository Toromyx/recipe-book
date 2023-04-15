import type { Readable } from "svelte/store";
import { writable } from "svelte/store";
import { invoke } from "../command/client.ts";
import { Command } from "../command/command.ts";
import { listen } from "../event/client.ts";
import { EventChannel } from "../event/event-channel.ts";
import type { Loadable } from "../util/loadable.ts";

async function getUnitList(): Promise<string[]> {
  return invoke(Command.UNIT_LIST_GET, undefined);
}

function createUnitList(): Readable<Loadable<string[]>> {
  const { subscribe, set } = writable<Loadable<string[]>>(undefined);

  const setUnitList = () => {
    void getUnitList().then((unitList) => set(unitList));
  };
  setUnitList();

  for (const channel of [
    EventChannel.ENTITY_ACTION_CREATED_RECIPE_INGREDIENT,
    EventChannel.ENTITY_ACTION_UPDATED_RECIPE_INGREDIENT,
    EventChannel.ENTITY_ACTION_DELETED_RECIPE_INGREDIENT,
  ]) {
    void listen(channel, () => {
      setUnitList();
    });
  }

  return {
    subscribe,
  };
}

/**
 * A readable store of the loadable unit list
 */
export const unitList = createUnitList();
