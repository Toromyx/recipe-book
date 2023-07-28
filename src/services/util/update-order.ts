import type { IdentifiableInterface } from "../../types/identifiable-interface.ts";
import type { SortableUpdateInterface } from "../../types/sortable-interface.ts";
import type { EntityRepository } from "../store/repository/entity-repository.ts";
import type { Loadable } from "./loadable.ts";
import { isLoading } from "./loadable.ts";

/**
 * Update the order of all entities of the repository in an identifier ist by their order in this identifier list.
 *
 * One id will be skipped.
 */
export function updateOrder(
  entityRepository: EntityRepository<
    never,
    never,
    IdentifiableInterface & SortableUpdateInterface,
    never,
    never
  >,
  idList: Loadable<number[]>,
  skipId: number,
): void {
  if (isLoading(idList)) {
    return;
  }
  const filteredList = idList.filter((id) => id !== skipId);
  for (let i = 0; i < filteredList.length; i++) {
    const id = filteredList[i];
    void entityRepository.update(id, () => ({
      id,
      order: i + 1,
    }));
  }
}
