import { Item } from "./Interfaces";

export const RECEIVE_ITEMS = "RECEIVE_ITEMS";

export interface ItemState {
  items: Item[],
}

interface ReceiveItemsAction {
  type: typeof RECEIVE_ITEMS,
  items: Item[],
}

export type ItemActionTypes = ReceiveItemsAction;
