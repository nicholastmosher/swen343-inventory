export const FETCH_ITEMS = "FETCH_ITEMS";
export const RECEIVE_ITEMS = "RECEIVE_ITEMS";

export interface Item {
  code: String,
  cost: number,
  description?: String,
}

export interface ItemState {
  items: Item[],
}

interface FetchItemsAction {
  type: typeof FETCH_ITEMS,
}

interface ReceiveItemsAction {
  type: typeof RECEIVE_ITEMS,
  items: Item[],
}

export type ItemActionTypes = FetchItemsAction | ReceiveItemsAction;
