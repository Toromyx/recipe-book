import type {
  FileCreateInterface,
  FileInterface,
  FileUpdateInterface,
} from "../../../types/entity/file-interface.ts";
import type {
  FileCondition,
  FileOrderBy,
} from "../../../types/filter/file-filter.ts";
import {
  countFile,
  createFile,
  deleteFile,
  listFile,
  readFile,
  updateFile,
} from "../../command/entity.ts";
import { listen } from "../../event/client.ts";
import { EventChannel } from "../../event/event-channel.ts";
import { EntityRepository } from "./entity-repository.ts";

export const fileRepository: EntityRepository<
  FileInterface,
  FileCreateInterface,
  FileUpdateInterface,
  FileCondition,
  FileOrderBy
> = new EntityRepository(
  (entityCreate) => createFile(entityCreate),
  (identifier) => readFile(identifier),
  (entityUpdate) => updateFile(entityUpdate),
  (identifier) => deleteFile(identifier),
  (filter) => listFile(filter),
  (condition) => countFile(condition),
  undefined,
  undefined,
  (reactFunction) => {
    void listen(EventChannel.ENTITY_ACTION_UPDATED_FILE, (event) => {
      reactFunction(event.payload);
    });
  },
  (reactFunction) => {
    void listen(EventChannel.ENTITY_ACTION_CREATED_FILE, () => {
      reactFunction();
    });
  },
  (reactFunction) => {
    void listen(EventChannel.ENTITY_ACTION_DELETED_FILE, (event) => {
      reactFunction(event.payload);
    });
  },
);
