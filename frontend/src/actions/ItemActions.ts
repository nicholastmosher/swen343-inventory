import { ThunkAction } from "redux-thunk";
import { AnyAction } from "redux";
import { Box, Item, Pallet, Warehouse } from "../types/Interfaces";
import {
  ItemActionTypes,
  RECEIVE_BOXES,
  RECEIVE_ITEMS,
  RECEIVE_PALLETS,
  RECEIVE_WAREHOUSES
} from "../types/ItemActionTypes";

import { BACKEND_URL } from "../config";

export const fetchWarehouses =
  (): ThunkAction<void, {}, {}, AnyAction> => async dispatch => {
    const warehouses: Warehouse[] = await fetch(`${BACKEND_URL}/api/v1/warehouses`)
      .then(res => res.json());
    dispatch(receiveWarehouses(warehouses))
  };

export const receiveWarehouses = (warehouses: Warehouse[]): ItemActionTypes => ({
  type: RECEIVE_WAREHOUSES,
  warehouses,
});

export const createWarehouse =
  (warehouse: Warehouse): ThunkAction<void, {}, {}, AnyAction> => async dispatch => {

    await fetch(`${BACKEND_URL}/api/v1/warehouses`, {
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

    const items: Item[] = await fetch(`${BACKEND_URL}/api/v1/items`)
      .then(res => res.json());

    dispatch(receiveItems(items));
  };

export const receiveItems = (items: Item[]): ItemActionTypes => ({
  type: RECEIVE_ITEMS,
  items,
});

export const createItem =
  (item: Item): ThunkAction<void, {}, {}, AnyAction> => async dispatch => {

  await fetch(`${BACKEND_URL}/api/v1/items`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(item),
  });

  dispatch(fetchItems());
};

export const fetchPallets =
  (): ThunkAction<void, {}, {}, AnyAction> => async dispatch => {

    const pallets: Pallet[] = await fetch(`${BACKEND_URL}/api/v1/pallets`)
      .then(res => res.json());

    dispatch(receivePallets(pallets));
  };

export const receivePallets = (pallets: Pallet[]): ItemActionTypes => ({
  type: RECEIVE_PALLETS,
  pallets,
});

export const createPallet =
  (pallet: Pallet): ThunkAction<void, {}, {}, AnyAction> => async dispatch => {

  await fetch(`${BACKEND_URL}/api/v1/pallets`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(pallet),
  });

  dispatch(fetchPallets());
};

export const fetchBoxes =
  (): ThunkAction<void, {}, {}, AnyAction> => async dispatch => {

    const boxes: Box[] = await fetch(`${BACKEND_URL}/api/v1/boxes`)
      .then(res => res.json());

    dispatch(receiveBoxes(boxes));
  };

export const receiveBoxes = (boxes: Box[]): ItemActionTypes => ({
  type: RECEIVE_BOXES,
  boxes,
});

export const createBox =
  (box: Box): ThunkAction<void, {}, {}, AnyAction> => async dispatch => {

    await fetch(`${BACKEND_URL}/api/v1/boxes`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(box),
    });

    dispatch(fetchBoxes());
  };
