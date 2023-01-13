import { equalArray } from "../util/compare.js";
import { IdentifiableInterface } from "../../types/identifiable-interface.js";

type EntityRepositorySubscriber<Entity> = (entity: Entity | undefined) => void;

export type EntityRepositoryListSubscriber = (list: number[]) => void;

export type EntityRepositoryCountSubscriber = (count: number) => void;

type EntityRepositoryUpdater<Entity> = (entity: Entity) => Entity;

export type EntityRepositoryUnsubscriber = () => void;

export interface EntityRepositoryInterface<
  Entity extends IdentifiableInterface,
  Filter extends object,
> {
  /**
   * subscribes to changes of the entity with the specified identifier
   *
   * always remember to unsubscribe when the subscription is not needed anymore
   *
   * @param identifier - identifies the entity
   * @param subscriber - this function is called on changes to the entity, with the entity as parameter
   *
   * @return EntityRepositoryUnsubscriber - the returned function must be called to unsubscribe from entity changes
   */
  subscribe(
    identifier: number,
    subscriber: EntityRepositorySubscriber<Entity>,
  ): EntityRepositoryUnsubscriber;

  /**
   * subscribes to changes of the complete entity list
   *
   * always remember to unsubscribe when the subscription is not needed anymore
   *
   * @param subscriber - this function is called on changes to the entity list, with the list as parameter
   *
   * @return EntityRepositoryUnsubscriber - the returned function must be called to unsubscribe from entity list changes
   */
  subscribeList(
    subscriber: EntityRepositoryListSubscriber,
  ): EntityRepositoryUnsubscriber;

  /**
   * subscribes to changes of the complete entity count
   *
   * always remember to unsubscribe when the subscription is not needed anymore
   *
   * @param subscriber - this function is called on changes to the entity count, with the count as parameter
   *
   * @return EntityRepositoryUnsubscriber - the returned function must be called to unsubscribe from entity count changes
   */
  subscribeCount(
    subscriber: EntityRepositoryCountSubscriber,
  ): EntityRepositoryUnsubscriber;

  /**
   * subscribes to changes of a filtered entity list
   *
   * always remember to unsubscribe when the subscription is not needed anymore
   *
   * @param filter - the list filter
   * @param subscriber - this function is called on changes to the filtered entity list, with the list as parameter
   *
   * @return EntityRepositoryUnsubscriber - the returned function must be called to unsubscribe from filtered entity list changes
   */
  subscribeListFiltered(
    filter: Filter,
    subscriber: EntityRepositoryListSubscriber,
  ): EntityRepositoryUnsubscriber;

  /**
   * subscribes to changes of a filtered entity count
   *
   * always remember to unsubscribe when the subscription is not needed anymore
   *
   * @param filter - the list filter
   * @param subscriber - this function is called on changes to the filtered entity count, with the count as parameter
   *
   * @return EntityRepositoryUnsubscriber - the returned function must be called to unsubscribe from filtered entity count changes
   */
  subscribeCountFiltered(
    filter: Filter,
    subscriber: EntityRepositoryCountSubscriber,
  ): EntityRepositoryUnsubscriber;

  /**
   * create an entity based on partial data via API
   *
   * adds the entity to the state
   *
   * @param partial - the partial entity data
   */
  create(partial: Partial<Entity>): Promise<number>;

  /**
   * call this method to update the entity with the specified identifier based on the previous entity
   *
   * the updater callback receives the current entity as parameter
   *
   * this method calls all subscribers after updating the entity
   *
   * @param identifier - identifies the entity
   * @param updater - is called with the current entity
   */
  update(
    identifier: number,
    updater: EntityRepositoryUpdater<Entity>,
  ): Promise<void>;

  /**
   * deletes the entity from the state and all its subscribers, and via API
   *
   * does not notify subscribers in any way
   *
   * @param identifier - identifies the entity
   */
  delete(identifier: number): Promise<void>;
}

export function stringifyFilter(filter: object): string {
  return JSON.stringify(filter, Object.keys(filter).sort());
}

export class EntityRepository<
  Entity extends IdentifiableInterface,
  Filter extends object,
> implements EntityRepositoryInterface<Entity, Filter>
{
  protected state: {
    [identifier: number]: Entity;
  } = {};

  protected listState: number[] = [];

  protected countState = NaN;

  protected filteredListState: {
    [filterKey: string]: number[];
  } = {};

  protected filteredCountState: {
    [filterKey: string]: number;
  } = {};

  protected subscribers: {
    [identifier: number]: Set<EntityRepositorySubscriber<Entity>>;
  } = {};

  protected listSubscribers: Set<EntityRepositoryListSubscriber> = new Set();

  protected countSubscribers: Set<EntityRepositoryCountSubscriber> = new Set();

  protected filteredListSubscribers: {
    [filterKey: string]: {
      filter: Filter;
      set: Set<EntityRepositoryListSubscriber>;
    };
  } = {};

  protected filteredCountSubscribers: {
    [filterKey: string]: {
      filter: Filter;
      set: Set<EntityRepositoryCountSubscriber>;
    };
  } = {};

  protected readonly apiRead: (identifier: number) => Promise<Entity>;

  protected readonly apiCreate: (partial: Partial<Entity>) => Promise<number>;

  protected readonly apiUpdate: (partial: Partial<Entity>) => Promise<void>;

  protected readonly apiDelete: (identifier: number) => Promise<void>;

  protected readonly apiList: (filter: Filter) => Promise<number[]>;

  protected readonly apiCount: (filter: Filter) => Promise<number>;

  protected readonly defaultFilter: Filter;

  /**
   * @param apiCreate - create a new entity via the API
   * @param apiRead - get the entity from the API
   * @param apiUpdate - update an existing entity via the API
   * @param apiDelete - delete an existing entity via the API
   * @param apiList - get a filtered list of entities from the API
   * @param apiCount - get a filtered count of entities from the API
   * @param defaultFilter - the default filter
   * @param registerUpdate - register callbacks for reacting to entity updates
   * @param registerCreate - register callbacks for reacting to entity creations
   * @param registerDelete - register callbacks for reacting to entity deletions
   */
  public constructor(
    apiCreate: (partial: Partial<Entity>) => Promise<number>,
    apiRead: (identifier: number) => Promise<Entity>,
    apiUpdate: (partial: Partial<Entity>) => Promise<void>,
    apiDelete: (identifier: number) => Promise<void>,
    apiList: (filter: Filter) => Promise<number[]>,
    apiCount: (filter: Filter) => Promise<number>,
    defaultFilter: Filter,
    registerUpdate: (reactFunction: (identifier: number) => void) => void,
    registerCreate: (reactFunction: () => void) => void,
    registerDelete: (reactFunction: (identifier: number) => void) => void,
  ) {
    this.apiCreate = apiCreate;
    this.apiRead = apiRead;
    this.apiUpdate = apiUpdate;
    this.apiDelete = apiDelete;
    this.apiList = apiList;
    this.apiCount = apiCount;
    this.defaultFilter = defaultFilter;

    const react = async (identifier: number): Promise<void> => {
      this.state[identifier] = await this.apiRead(identifier);
      this.run(identifier);
    };

    const reactDelete = (identifier: number): void => {
      delete this.state[identifier];
      this.run(identifier);
      delete this.subscribers[identifier];
    };

    const reactList = async (): Promise<void> => {
      const list = await this.apiList(this.defaultFilter);
      if (!equalArray(this.listState, list)) {
        this.listState = list;
        this.runList();
      }
    };
    const reactCount = async (): Promise<void> => {
      const count = await this.apiCount(this.defaultFilter);
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
        const filterKey = stringifyFilter(filteredCountSubscriber.filter);
        const count = await this.apiCount(filteredCountSubscriber.filter);
        if (this.filteredCountState[filterKey] !== count) {
          this.filteredCountState[filterKey] = count;
          this.runCountFiltered(filterKey);
        }
      }
    };

    registerUpdate((identifier: number) => {
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

    registerDelete((identifier: number) => {
      reactDelete(identifier);
      void reactList();
      void reactCount();
      void reactFilteredList();
      void reactFilteredCount();
    });
  }

  public subscribe(
    identifier: number,
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
      this.subscribers[identifier]?.delete(subscriber);
    };
  }

  public subscribeList(
    subscriber: EntityRepositoryListSubscriber,
  ): EntityRepositoryUnsubscriber {
    void this.list().then(() => {
      subscriber(this.listState);
    });
    this.listSubscribers.add(subscriber);

    return (): void => {
      this.listSubscribers.delete(subscriber);
    };
  }

  public subscribeCount(
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

  public subscribeListFiltered(
    filter: Filter,
    subscriber: EntityRepositoryListSubscriber,
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
      this.filteredListSubscribers[filterKey]?.set.delete(subscriber);
    };
  }

  public subscribeCountFiltered(
    filter: Filter,
    subscriber: EntityRepositoryCountSubscriber,
  ): EntityRepositoryUnsubscriber {
    const filterKey = stringifyFilter(filter);
    void this.countFiltered(filter).then(() => {
      subscriber(this.filteredCountState[filterKey]);
    });

    if (!this.filteredCountSubscribers[filterKey]) {
      this.filteredCountSubscribers[filterKey] = {
        filter,
        set: new Set(),
      };
    }
    this.filteredCountSubscribers[filterKey]?.set.add(subscriber);

    return (): void => {
      this.filteredCountSubscribers[filterKey]?.set.delete(subscriber);
    };
  }

  public async create(partial: Partial<Entity>): Promise<number> {
    return this.apiCreate(partial);
  }

  public async update(
    identifier: number,
    updater: EntityRepositoryUpdater<Entity>,
  ): Promise<void> {
    const entity = this.state[identifier];
    if (!entity) {
      return;
    }
    return this.apiUpdate(updater(entity));
  }

  public async delete(identifier: number): Promise<void> {
    await this.apiDelete(identifier);
  }

  /**
   * calls current subscribers of the entity with the specified identifier
   *
   * this method is to be called after the state of that entity has changed
   */
  protected run(identifier: number): void {
    const entity = this.state[identifier];
    this.subscribers[identifier]?.forEach((subscriber) => subscriber(entity));
  }

  /**
   * calls current subscribers of the entity list
   *
   * this method is to be called after change of the list state
   */
  protected runList(): void {
    this.listSubscribers.forEach((subscriber) => subscriber(this.listState));
  }

  /**
   * calls current subscribers of the entity count
   *
   * this method is to be called after change of the count state
   */
  protected runCount(): void {
    this.countSubscribers.forEach((subscriber) => subscriber(this.countState));
  }

  /**
   * calls current subscribers of a filtered entity list
   *
   * this method is to be called after change of the filtered list state
   */
  protected runListFiltered(filterKey: string): void {
    const list = this.filteredListState[filterKey];
    if (list) {
      this.filteredListSubscribers[filterKey]?.set.forEach((subscriber) =>
        subscriber(list),
      );
    }
  }

  /**
   * calls current subscribers of a filtered entity count
   *
   * this method is to be called after change of the filtered count state
   */
  protected runCountFiltered(filterKey: string): void {
    const count = this.filteredCountState[filterKey];
    if (count !== undefined) {
      this.filteredCountSubscribers[filterKey]?.set.forEach((subscriber) =>
        subscriber(count),
      );
    }
  }

  /**
   * loads the entity and adds it to the state if not already present
   */
  private async read(identifier: number): Promise<void> {
    if (!this.state[identifier]) {
      this.state[identifier] = await this.apiRead(identifier);
    }
  }

  /**
   * loads the entity list and sets the state if not already present
   */
  private async list(): Promise<void> {
    if (!this.listState.length) {
      this.listState = await this.apiList(this.defaultFilter);
    }
  }

  /**
   * loads the entity count and sets the count state
   */
  private async count(): Promise<void> {
    if (Number.isNaN(this.countState)) {
      this.countState = await this.apiCount(this.defaultFilter);
    }
  }

  /**
   * loads a filtered entity list and adds them to the state and filtered state if not already present
   */
  private async listFiltered(filter: Filter): Promise<void> {
    const filterKey = stringifyFilter(filter);
    if (!this.filteredListState[filterKey]) {
      this.filteredListState[filterKey] = await this.apiList(filter);
    }
  }

  /**
   * loads a filtered entity count and sets the filtered count state if not already present
   */
  private async countFiltered(filter: Filter): Promise<void> {
    const filterKey = stringifyFilter(filter);
    if (!this.filteredCountState[filterKey]) {
      this.filteredCountState[filterKey] = await this.apiCount(filter);
    }
  }
}
