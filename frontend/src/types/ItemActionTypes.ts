export const RECEIVE_ITEMS = "RECEIVE_ITEMS";

export interface Item {
  code: String,
  cost: number,
  description?: String,
}

export interface ItemState {
  items: Item[],
}

interface ReceiveItemsAction {
  type: typeof RECEIVE_ITEMS,
  items: Item[],
}

export type ItemActionTypes = ReceiveItemsAction;
