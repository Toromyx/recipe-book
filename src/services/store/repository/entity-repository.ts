import type { Readable } from "svelte/store";
import type { FilterInterface } from "../../../types/filter-interface.ts";
import type { IdentifiableInterface } from "../../../types/identifiable-interface.ts";
import { equalArray } from "../../util/compare.ts";
import type { Loadable } from "../../util/loadable.ts";

type ApiCreate<EntityCreate, PrimaryKey> = (
  entityCreate: EntityCreate,
) => Promise<PrimaryKey>;

type ApiRead<Entity, PrimaryKey> = (identifier: PrimaryKey) => Promise<Entity>;

type ApiUpdate<EntityUpdate> = (entityUpdate: EntityUpdate) => Promise<void>;

type ApiDelete<PrimaryKey> = (identifier: PrimaryKey) => Promise<void>;

type ApiList<Filter, PrimaryKey> = (filter: Filter) => Promise<PrimaryKey[]>;

type ApiCount<Condition> = (condition?: Condition) => Promise<number>;

type EntityRepositorySubscriber<Entity> = (entity: Entity | undefined) => void;

type EntityRepositoryListSubscriber<PrimaryKey> = (list: PrimaryKey[]) => void;

type EntityRepositoryCountSubscriber = (count: number) => void;

type EntityRepositoryUpdater<Entity, EntityUpdate> = (
  entity: Entity,
) => EntityUpdate;

type EntityRepositoryUnsubscriber = () => void;

const DELETE_DELAY = 100_000;

/**
 * Stringify a filter object in JSON.
 *
 * This is achieved by first recursively getting all its keys via JSON.stringify and then JSON-stringifying it again with the sorted keys.
 * @param filter
 */
export function stringifyFilter(filter: unknown): string {
  const allKeys: Set<string> = new Set();
  JSON.stringify(filter, (key, value) => {
    allKeys.add(key);
    // eslint-disable-next-line @typescript-eslint/no-unsafe-return
    return value;
  });
  return JSON.stringify(filter, [...allKeys].sort());
}

/**
 * This class implements a reactive entity repository for CRUD+L+Count operations.
 *
 * It provides methods to create a reactive store for a specific entity, the entity list or count, and a filtered entity list or count.
 * Ths entity list is always an array of entity identifiers. The entity data itself needs to be subscribed to separately.
 */
export class EntityRepository<
  Entity extends IdentifiableInterface,
  EntityCreate extends object,
  EntityUpdate extends IdentifiableInterface,
  Condition extends object,
  OrderBy extends object,
  PrimaryKey extends number | string = number,
> {
  /**
   * The state contains the entity data, keyed by their identifier.
   */
  state: {
    [identifier in PrimaryKey]?: Entity;
  } = {};

  /**
   * The list state contains a list of all entity identifiers.
   */
  listState: PrimaryKey[] = [];

  /**
   * The count state contains the count of all entities.
   */
  countState = NaN;

  /**
   * The filtered list state contains multiple lists of entity identifiers, keyed by their corresponding filter.
   */
  filteredListState: {
    [filterKey: string]: PrimaryKey[];
  } = {};

  /**
   * A collection of timeout ids whose timeouts are currently running to delete an entry in the filtered list state.
   */
  filteredListStateDeleteTimeouts: {
    [filterKey: string]: number;
  } = {};

  /**
   * The filtered count state contains multiple entity counts, keyed by their corresponding filter.
   */
  filteredCountState: {
    [conditionKey: string]: number;
  } = {};

  /**
   * A collection of timeout ids whose timeouts are currently running to delete an entry in the filtered count state.
   */
  filteredCountStateDeleteTimeouts: {
    [conditionKey: string]: number;
  } = {};

  /**
   * This object contains a set of active subscriber functions for each entity, keyed by the entity identifier.
   */
  subscribers: {
    [identifier in PrimaryKey]?: Set<EntityRepositorySubscriber<Entity>>;
  } = {};

  /**
   * This set contains all active list subscriber functions.
   */
  listSubscribers: Set<EntityRepositoryListSubscriber<PrimaryKey>> = new Set();

  /**
   * This set contains all active count subscriber functions.
   */
  countSubscribers: Set<EntityRepositoryCountSubscriber> = new Set();

  /**
   * This object contains a set of active filtered list subscriber functions for each filter in use.
   */
  filteredListSubscribers: {
    [filterKey: string]: {
      filter: FilterInterface<Condition, OrderBy>;
      set: Set<EntityRepositoryListSubscriber<PrimaryKey>>;
    };
  } = {};

  /**
   * This object contains a set of active filtered count subscriber functions for each filter in use.
   */
  filteredCountSubscribers: {
    [conditionKey: string]: {
      condition?: Condition;
      set: Set<EntityRepositoryCountSubscriber>;
    };
  } = {};

  /**
   * Read an entity via the API.
   */
  apiRead: ApiRead<Entity, PrimaryKey>;

  /**
   * Create an entity via the API.
   */
  apiCreate: ApiCreate<EntityCreate, PrimaryKey>;

  /**
   * Update an entity via the API.
   */
  apiUpdate: ApiUpdate<EntityUpdate>;

  /**
   * Delete an entity via the API.
   */
  apiDelete: ApiDelete<PrimaryKey>;

  /**
   * List entities via the API.
   */
  apiList: ApiList<FilterInterface<Condition, OrderBy>, PrimaryKey>;

  /**
   * Count entities via the API.
   */
  apiCount: ApiCount<Condition>;

  /**
   * The default condition is used for the unfiltered list and count state.
   */
  defaultCondition: Condition | undefined;

  /**
   * The default order-by is used for the unfiltered list state.
   */
  defaultOrderBy: OrderBy[] | undefined;

  /**
   * The default filter is used for the unfiltered list state.
   * It is built from the {@see defaultCondition} and {@see defaultOrderBy}.
   */
  defaultFilter: FilterInterface<Condition, OrderBy>;

  /**
   * Construct an entity repository for a specific entity and a specific API implementation.
   *
   * @param apiCreate - {@see apiCreate}
   * @param apiRead - {@see apiRead}
   * @param apiUpdate - {@see apiUpdate}
   * @param apiDelete - {@see apiDelete}
   * @param apiList - {@see apiList}
   * @param apiCount - {@see apiCount}
   * @param defaultCondition - {@see defaultCondition}
   * @param defaultOrderBy - {@see defaultOrderBy}
   * @param registerUpdate - register callbacks for reacting to entity updates
   * @param registerCreate - register callbacks for reacting to entity creations
   * @param registerDelete - register callbacks for reacting to entity deletions
   * @param registerFilterRelatedActions - register callbacks for reacting to related entity actions which might influence filtered lists and counts
   */
  constructor(
    apiCreate: ApiCreate<EntityCreate, PrimaryKey>,
    apiRead: ApiRead<Entity, PrimaryKey>,
    apiUpdate: ApiUpdate<EntityUpdate>,
    apiDelete: ApiDelete<PrimaryKey>,
    apiList: ApiList<FilterInterface<Condition, OrderBy>, PrimaryKey>,
    apiCount: ApiCount<Condition>,
    defaultCondition: Condition | undefined,
    defaultOrderBy: OrderBy[] | undefined,
    registerUpdate: (reactFunction: (identifier: PrimaryKey) => void) => void,
    registerCreate: (reactFunction: () => void) => void,
    registerDelete: (reactFunction: (identifier: PrimaryKey) => void) => void,
    registerFilterRelatedActions?: (reactFunction: () => void) => void,
  ) {
    this.apiCreate = apiCreate;
    this.apiRead = apiRead;
    this.apiUpdate = apiUpdate;
    this.apiDelete = apiDelete;
    this.apiList = apiList;
    this.apiCount = apiCount;
    this.defaultCondition = defaultCondition;
    this.defaultOrderBy = defaultOrderBy;
    this.defaultFilter = {
      condition: this.defaultCondition,
      orderBy: this.defaultOrderBy,
    };

    const react = async (identifier: PrimaryKey): Promise<void> => {
      if (!this.state[identifier]) {
        return;
      }
      this.state[identifier] = await this.apiRead(identifier);
      this.run(identifier);
    };

    const reactDelete = (identifier: PrimaryKey): void => {
      if (!this.state[identifier]) {
        return;
      }
      delete this.state[identifier];
      this.run(identifier);
      delete this.subscribers[identifier];
    };

    const reactList = async (): Promise<void> => {
      if (!this.listSubscribers.size) {
        return;
      }
      const list = await this.apiList(this.defaultFilter);
      if (!equalArray(this.listState, list)) {
        this.listState = list;
        this.runList();
      }
    };

    const reactCount = async (): Promise<void> => {
      if (!this.countSubscribers.size) {
        return;
      }
      const count = await this.apiCount(this.defaultCondition);
      if (this.countState !== count) {
        this.countState = count;
        this.runCount();
      }
    };

    const reactFilteredList = async (): Promise<void> => {
      for (const filteredListSubscriber of Object.values(
        this.filteredListSubscribers,
      )) {
        const filterKey = stringifyFilter(filteredListSubscriber.filter);
        const list = await this.apiList(filteredListSubscriber.filter);
        if (!equalArray(this.filteredListState[filterKey], list)) {
          this.filteredListState[filterKey] = list;
          this.runListFiltered(filterKey);
        }
      }
    };

    const reactFilteredCount = async (): Promise<void> => {
      for (const filteredCountSubscriber of Object.values(
        this.filteredCountSubscribers,
      )) {
        const conditionKey = stringifyFilter(filteredCountSubscriber.condition);
        const count = await this.apiCount(filteredCountSubscriber.condition);
        if (this.filteredCountState[conditionKey] !== count) {
          this.filteredCountState[conditionKey] = count;
          this.runCountFiltered(conditionKey);
        }
      }
    };

    registerUpdate((identifier: PrimaryKey) => {
      void react(identifier);
      void reactFilteredList();
      void reactFilteredCount();
    });

    registerCreate(() => {
      void reactList();
      void reactCount();
      void reactFilteredList();
      void reactFilteredCount();
    });

    registerDelete((identifier: PrimaryKey) => {
      reactDelete(identifier);
      void reactList();
      void reactCount();
      void reactFilteredList();
      void reactFilteredCount();
    });

    if (registerFilterRelatedActions) {
      registerFilterRelatedActions(() => {
        void reactFilteredList();
        void reactFilteredCount();
      });
    }
  }

  /**
   * Subscribe to changes of the entity with the specified identifier.
   *
   * Always remember to unsubscribe when the subscription is not needed anymore!
   *
   * @param identifier - identifies the entity
   * @param subscriber - this function is called on changes to the entity, with the entity as parameter
   *
   * @return EntityRepositoryUnsubscriber - the returned function must be called to unsubscribe from entity changes
   */
  subscribe(
    identifier: PrimaryKey,
    subscriber: EntityRepositorySubscriber<Entity>,
  ): EntityRepositoryUnsubscriber {
    void this.read(identifier).then(() => {
      subscriber(this.state[identifier]);
    });

    if (!this.subscribers[identifier]) {
      this.subscribers[identifier] = new Set();
    }
    this.subscribers[identifier]?.add(subscriber);

    return (): void => {
      const set = this.subscribers[identifier];
      if (!set) {
        return;
      }
      set.delete(subscriber);
      if (!set.size) {
        delete this.subscribers[identifier];
      }
    };
  }

  /**
   * Create a reactive store of a single entity.
   */
  createStore(identifier: PrimaryKey): Readable<Loadable<Entity>> {
    return {
      subscribe: (subscriber) =>
        this.subscribe(identifier, (entity) => {
          subscriber(entity);
        }),
    };
  }

  /**
   * Subscribe to changes of the complete entity list.
   *
   * Always remember to unsubscribe when the subscription is not needed anymore.
   *
   * @param subscriber - this function is called on changes to the entity list, with the list as parameter
   *
   * @return EntityRepositoryUnsubscriber - the returned function must be called to unsubscribe from entity list changes
   */
  subscribeList(
    subscriber: EntityRepositoryListSubscriber<PrimaryKey>,
  ): EntityRepositoryUnsubscriber {
    void this.list().then(() => {
      subscriber(this.listState);
    });
    this.listSubscribers.add(subscriber);

    return (): void => {
      this.listSubscribers.delete(subscriber);
    };
  }

  /**
   * Create a reactive store of the complete entity list.
   */
  createListStore(): Readable<Loadable<PrimaryKey[]>> {
    return {
      subscribe: (subscriber) =>
        this.subscribeList((list) => {
          subscriber(list);
        }),
    };
  }

  /**
   * Subscribe to changes of the complete entity count.
   *
   * Always remember to unsubscribe when the subscription is not needed anymore.
   *
   * @param subscriber - this function is called on changes to the entity count, with the count as parameter
   *
   * @return EntityRepositoryUnsubscriber - the returned function must be called to unsubscribe from entity count changes
   */
  subscribeCount(
    subscriber: EntityRepositoryCountSubscriber,
  ): EntityRepositoryUnsubscriber {
    void this.count().then(() => {
      subscriber(this.countState);
    });
    this.countSubscribers.add(subscriber);

    return (): void => {
      this.countSubscribers.delete(subscriber);
    };
  }

  /**
   * Create a reactive store of the complete entity count.
   */
  createCountStore(): Readable<Loadable<number>> {
    return {
      subscribe: (subscriber) =>
        this.subscribeCount((count) => {
          subscriber(count);
        }),
    };
  }

  /**
   * Subscribe to changes of a filtered entity list.
   *
   * Always remember to unsubscribe when the subscription is not needed anymore.
   *
   * @param filter - the list filter
   * @param subscriber - this function is called on changes to the filtered entity list, with the list as parameter
   *
   * @return EntityRepositoryUnsubscriber - the returned function must be called to unsubscribe from filtered entity list changes
   */
  subscribeListFiltered(
    filter: FilterInterface<Condition, OrderBy>,
    subscriber: EntityRepositoryListSubscriber<PrimaryKey>,
  ): EntityRepositoryUnsubscriber {
    const filterKey = stringifyFilter(filter);
    void this.listFiltered(filter).then(() => {
      subscriber(this.filteredListState[filterKey]);
    });

    if (!this.filteredListSubscribers[filterKey]) {
      this.filteredListSubscribers[filterKey] = {
        filter,
        set: new Set(),
      };
    }
    this.filteredListSubscribers[filterKey]?.set.add(subscriber);

    return (): void => {
      const set = this.filteredListSubscribers[filterKey]?.set;
      if (!set) {
        return;
      }
      set.delete(subscriber);
      if (!set.size) {
        delete this.filteredListSubscribers[filterKey];
        if (this.filteredListStateDeleteTimeouts[filterKey]) {
          clearTimeout(this.filteredListStateDeleteTimeouts[filterKey]);
          delete this.filteredListStateDeleteTimeouts[filterKey];
        }
        this.filteredListStateDeleteTimeouts[filterKey] = setTimeout(() => {
          delete this.filteredListState[filterKey];
          delete this.filteredListStateDeleteTimeouts[filterKey];
        }, DELETE_DELAY);
      }
    };
  }

  /**
   * Create a reactive store of a filtered entity list.
   */
  createListFilteredStore(
    filter: FilterInterface<Condition, OrderBy>,
  ): Readable<Loadable<PrimaryKey[]>> {
    return {
      subscribe: (subscriber) =>
        this.subscribeListFiltered(filter, (list) => {
          subscriber(list);
        }),
    };
  }

  /**
   * Subscribe to changes of a filtered entity count.
   *
   * Always remember to unsubscribe when the subscription is not needed anymore.
   *
   * @param condition - the count condition
   * @param subscriber - this function is called on changes to the filtered entity count, with the count as parameter
   *
   * @return EntityRepositoryUnsubscriber - the returned function must be called to unsubscribe from filtered entity count changes
   */
  subscribeCountFiltered(
    condition: Condition | undefined,
    subscriber: EntityRepositoryCountSubscriber,
  ): EntityRepositoryUnsubscriber {
    const conditionKey = stringifyFilter(condition);
    void this.countFiltered(condition).then(() => {
      subscriber(this.filteredCountState[conditionKey]);
    });

    if (!this.filteredCountSubscribers[conditionKey]) {
      this.filteredCountSubscribers[conditionKey] = {
        condition,
        set: new Set(),
      };
    }
    this.filteredCountSubscribers[conditionKey]?.set.add(subscriber);

    return (): void => {
      const set = this.filteredCountSubscribers[conditionKey]?.set;
      if (!set) {
        return;
      }
      set.delete(subscriber);
      if (!set.size) {
        delete this.filteredCountSubscribers[conditionKey];
        if (this.filteredCountStateDeleteTimeouts[conditionKey]) {
          clearTimeout(this.filteredCountStateDeleteTimeouts[conditionKey]);
          delete this.filteredCountStateDeleteTimeouts[conditionKey];
        }
        this.filteredCountStateDeleteTimeouts[conditionKey] = setTimeout(() => {
          delete this.filteredCountState[conditionKey];
          delete this.filteredCountStateDeleteTimeouts[conditionKey];
        }, DELETE_DELAY);
      }
    };
  }

  /**
   * Create a reactive store of a filtered entity count.
   */
  createCountFilteredStore(condition?: Condition): Readable<Loadable<number>> {
    return {
      subscribe: (subscriber) =>
        this.subscribeCountFiltered(condition, (count) => {
          subscriber(count);
        }),
    };
  }

  /**
   * Create an entity based on partial data via API.
   *
   * @param entityCreate - the entity data for creation
   */
  async create(entityCreate: EntityCreate): Promise<PrimaryKey> {
    return this.apiCreate(entityCreate);
  }

  /**
   * Update the entity with the specified identifier based on the previous entity.
   *
   * The updater callback receives the current entity as parameter.
   *
   * @param identifier - identifies the entity
   * @param updater - is called with the current entity
   */
  async update(
    identifier: PrimaryKey,
    updater: EntityRepositoryUpdater<Entity, EntityUpdate>,
  ): Promise<void> {
    const entity = this.state[identifier];
    if (!entity) {
      return;
    }
    return this.apiUpdate(updater(entity));
  }

  /**
   * Delete the entity via API.
   *
   * @param identifier - identifies the entity
   */
  async delete(identifier: PrimaryKey): Promise<void> {
    await this.apiDelete(identifier);
  }

  /**
   * Call active subscribers of the entity with the specified identifier.
   *
   * This method is called after the state of that entity has changed.
   */
  run(identifier: PrimaryKey): void {
    const entity = this.state[identifier];
    this.subscribers[identifier]?.forEach((subscriber) => subscriber(entity));
  }

  /**
   * Calls active subscribers of the complete entity list.
   *
   * This method is called after change to the list state.
   */
  runList(): void {
    this.listSubscribers.forEach((subscriber) => subscriber(this.listState));
  }

  /**
   * Call active subscribers of the complete entity count.
   *
   * This method is called after change to the count state.
   */
  runCount(): void {
    this.countSubscribers.forEach((subscriber) => subscriber(this.countState));
  }

  /**
   * Call active subscribers of a filtered entity list.
   *
   * This method is called after change to the filtered list state with the specified filter.
   */
  runListFiltered(filterKey: string): void {
    const list = this.filteredListState[filterKey];
    if (list) {
      this.filteredListSubscribers[filterKey]?.set.forEach((subscriber) =>
        subscriber(list),
      );
    }
  }

  /**
   * Call active subscribers of a filtered entity count.
   *
   * This method is called after change to the filtered count state with the specified filter.
   */
  runCountFiltered(conditionKey: string): void {
    const count = this.filteredCountState[conditionKey];
    if (count !== undefined) {
      this.filteredCountSubscribers[conditionKey]?.set.forEach((subscriber) =>
        subscriber(count),
      );
    }
  }

  /**
   * Read the entity via the API and add it to the state if it is not already present.
   */
  async read(identifier: PrimaryKey): Promise<void> {
    if (!this.state[identifier]) {
      this.state[identifier] = await this.apiRead(identifier).catch(
        (reason) => {
          if (reason === "NotFound") {
            return undefined;
          } else {
            throw reason;
          }
        },
      );
    }
  }

  /**
   * Read the complete entity list and set the list state if not already set.
   */
  async list(): Promise<void> {
    if (!this.listState.length) {
      this.listState = await this.apiList(this.defaultFilter);
    }
  }

  /**
   * Read the complete entity count and set the count state if not already set.
   */
  async count(): Promise<void> {
    if (Number.isNaN(this.countState)) {
      this.countState = await this.apiCount(this.defaultCondition);
    }
  }

  /**
   * Read a filtered entity list and add it to the filtered list state if not already present.
   */
  async listFiltered(
    filter: FilterInterface<Condition, OrderBy>,
  ): Promise<void> {
    const filterKey = stringifyFilter(filter);
    if (this.filteredListStateDeleteTimeouts[filterKey]) {
      clearTimeout(this.filteredListStateDeleteTimeouts[filterKey]);
      delete this.filteredListStateDeleteTimeouts[filterKey];
    }
    if (!this.filteredListState[filterKey]) {
      this.filteredListState[filterKey] = await this.apiList(filter);
    }
  }

  /**
   * Read a filtered entity count and add it to the filtered count state if not already present.
   */
  async countFiltered(condition?: Condition): Promise<void> {
    const conditionKey = stringifyFilter(condition);
    if (this.filteredCountStateDeleteTimeouts[conditionKey]) {
      clearTimeout(this.filteredCountStateDeleteTimeouts[conditionKey]);
      delete this.filteredCountStateDeleteTimeouts[conditionKey];
    }
    if (!this.filteredCountState[conditionKey]) {
      this.filteredCountState[conditionKey] = await this.apiCount(condition);
    }
  }
}
