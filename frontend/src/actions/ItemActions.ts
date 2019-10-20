import { ThunkAction } from "redux-thunk";
import { AnyAction } from "redux";
import { ItemActionTypes, RECEIVE_ITEMS } from "../types/ItemActionTypes";
import axios from "axios";
import { Item } from "../types/Interfaces";

const host = "http://ec2-3-16-181-169.us-east-2.compute.amazonaws.com:8000";

export const fetchItems =
  (): ThunkAction<void, {}, {}, AnyAction> => async dispatch => {

    axios.get(`${host}/api/v1/items`, {
      headers: {
        "Content-Type": "application/json",
        "Access-Control-Allow-Origin": host, // axios might need this
      }
    })
      .then(res => console.log(res.data))
      .catch(err => console.log(err))

  };

export const receiveItems = (items: Item[]): ItemActionTypes => ({
  type: RECEIVE_ITEMS,
  items,
});
