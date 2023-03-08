import type { IdentifiableInterface } from "../../types/identifiable-interface.ts";
import type { Loadable } from "../../types/loadable.ts";
import type { SortableUpdateInterface } from "../../types/sortable-interface.ts";
import type { EntityRepositoryInterface } from "../repository/entity-repository.ts";
import { isLoading } from "./is-loading.ts";

export function updateOrder(
  entityRepository: EntityRepositoryInterface<
    never,
    never,
    IdentifiableInterface & SortableUpdateInterface,
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
