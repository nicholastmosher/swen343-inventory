import {ThunkAction} from "redux-thunk";
import {AnyAction} from "redux";
import {Item, ItemActionTypes, RECEIVE_ITEMS} from "../types/ItemActionTypes";
import axios from "axios";

const host = "ec2-3-16-181-169.us-east-2.compute.amazonaws.com:8000";

export const fetchItems =
  (): ThunkAction<void, {}, {}, AnyAction> => async dispatch => {

  const value = axios.get(`${host}/api/v1/items`, {
    headers: { "Content-Type": "application/json" }
  })
  .then(res => res.data)
  .catch(e => console.log(e));

  console.log(value);
};

export const receiveItems = (items: Item[]): ItemActionTypes => ({
  type: RECEIVE_ITEMS,
  items,
});
