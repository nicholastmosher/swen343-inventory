import {ThunkAction} from "redux-thunk";
import {AnyAction} from "redux";
import {Box, Item, Pallet, Warehouse} from "../types/Interfaces";
import {
  ItemActionTypes,
  RECEIVE_BOXES,
  RECEIVE_ITEMS,
  RECEIVE_PALLETS,
  RECEIVE_WAREHOUSES
} from "../types/ItemActionTypes";

const host = "http://localhost:8000";

export const fetchWarehouses =
  (): ThunkAction<void, {}, {}, AnyAction> => async dispatch => {

  const warehouses: Warehouse[] = await fetch(`${host}/api/v1/warehouses`)
    .then(res => res.json());

  dispatch(receiveWarehouses(warehouses))
};

export const receiveWarehouses = (warehouses: Warehouse[]): ItemActionTypes => ({
  type: RECEIVE_WAREHOUSES,
  warehouses,
});

export const insertWarehouse =
  (warehouse: Warehouse): ThunkAction<void, {}, {}, AnyAction> => async dispatch => {

  await fetch(`${host}/api/v1/warehouses`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(warehouse),
  }).then(res => res.json());

  dispatch(fetchWarehouses());
};

export const fetchItems =
  (): ThunkAction<void, {}, {}, AnyAction> => async dispatch => {

  const items: Item[] = await fetch(`${host}/api/v1/items`)
    .then(res => res.json());

  dispatch(receiveItems(items));
};

export const receiveItems = (items: Item[]): ItemActionTypes => ({
  type: RECEIVE_ITEMS,
  items,
});

export const fetchPallets =
  (): ThunkAction<void, {}, {}, AnyAction> => async dispatch => {

  const pallets: Pallet[] = await fetch(`${host}/api/v1/pallets`)
    .then(res => res.json());

  dispatch(receivePallets(pallets));
};

export const receivePallets = (pallets: Pallet[]): ItemActionTypes => ({
  type: RECEIVE_PALLETS,
  pallets,
});

export const fetchBoxes =
  (): ThunkAction<void, {}, {}, AnyAction> => async dispatch => {

  const boxes: Box[] = await fetch(`${host}/api/v1/boxes`)
    .then(res => res.json());

  dispatch(receiveBoxes(boxes));
};

export const receiveBoxes = (boxes: Box[]): ItemActionTypes => ({
  type: RECEIVE_BOXES,
  boxes,
});
