import type {
  UnitNameCreateInterface,
  UnitNameInterface,
  UnitNameUpdateInterface,
} from "../../../types/entity/unit-name-interface.ts";
import type { UnitNameFilterInterface } from "../../../types/filter/unit-name-filter-interface.ts";
import {
  countUnitName,
  createUnitName,
  deleteUnitName,
  listUnitName,
  readUnitName,
  updateUnitName,
} from "../../command/entity.ts";
import { listen } from "../../event/client.ts";
import { EventChannel } from "../../event/event-channel.ts";
import { EntityRepository } from "./entity-repository.ts";

export const unitNameRepository: EntityRepository<
  UnitNameInterface,
  UnitNameCreateInterface,
  UnitNameUpdateInterface,
  UnitNameFilterInterface
> = new EntityRepository(
  (entityCreate) => createUnitName(entityCreate),
  (identifier) => readUnitName(identifier),
  (entityUpdate) => updateUnitName(entityUpdate),
  (identifier) => deleteUnitName(identifier),
  (filter) => listUnitName(filter),
  (filter) => countUnitName(filter),
  {},
  (reactFunction) => {
    void listen(EventChannel.ENTITY_ACTION_UPDATED_UNIT_NAME, (event) => {
      reactFunction(event.payload);
    });
  },
  (reactFunction) => {
    void listen(EventChannel.ENTITY_ACTION_CREATED_UNIT_NAME, () => {
      reactFunction();
    });
  },
  (reactFunction) => {
    void listen(EventChannel.ENTITY_ACTION_DELETED_UNIT_NAME, (event) => {
      reactFunction(event.payload);
    });
  },
);
