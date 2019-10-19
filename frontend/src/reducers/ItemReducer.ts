import {ItemActionTypes, ItemState, RECEIVE_ITEMS} from "../types/ItemActionTypes";

const initialState: ItemState = {
  items: [],
};

export const ItemReducer = (state = initialState, action: ItemActionTypes) => {
  switch (action.type) {
    case RECEIVE_ITEMS:
      return { items: action.items };
    default:
      return state;
  }
};
