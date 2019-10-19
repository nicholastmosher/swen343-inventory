import {ThunkAction, ThunkDispatch} from "redux-thunk";
import {AnyAction} from "redux";
import {
  Item,
  FETCH_ITEMS,
  RECEIVE_ITEMS,
  ItemActionTypes,
} from "../types/ItemActionTypes";

const host = "blah";

export const fetchItems =
  (): ThunkAction<void, {}, {}, AnyAction> => async dispatch => {

};
