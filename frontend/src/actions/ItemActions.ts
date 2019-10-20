import { ThunkAction } from "redux-thunk";
import { AnyAction } from "redux";
import { ItemActionTypes, RECEIVE_ITEMS } from "../types/ItemActionTypes";
import { Item } from "../types/Interfaces";

const host = "http://localhost:8000";

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
