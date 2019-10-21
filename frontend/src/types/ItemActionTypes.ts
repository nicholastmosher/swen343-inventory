import {Box, Item, Pallet, Warehouse} from "./Interfaces";

export const RECEIVE_WAREHOUSES = "RECEIVE_WAREHOUSES";
export const RECEIVE_ITEMS = "RECEIVE_ITEMS";
export const RECEIVE_PALLETS = "RECEIVE_PALLETS";
export const RECEIVE_BOXES = "RECEIVE_BOXES";

export interface ItemState {
  warehouses: Warehouse[],
  items: Item[];
  pallets: Pallet[],
  boxes: Box[],
}

interface ReceiveWarehousesAction {
  type: typeof RECEIVE_WAREHOUSES,
  warehouses: Warehouse[],
}

interface ReceiveItemsAction {
  type: typeof RECEIVE_ITEMS,
  items: Item[],
}

interface ReceivePalletsAction {
  type: typeof RECEIVE_PALLETS,
  pallets: Pallet[],
}

interface ReceiveBoxesAction {
  type: typeof RECEIVE_BOXES,
  boxes: Box[],
}

export type ItemActionTypes =
  ReceiveWarehousesAction |
  ReceiveItemsAction |
  ReceivePalletsAction |
  ReceiveBoxesAction;
