import {
  ItemActionTypes,
  ItemState,
  RECEIVE_BOXES,
  RECEIVE_ITEMS,
  RECEIVE_PALLETS,
  RECEIVE_WAREHOUSES
} from "../types/ItemActionTypes";

const initialState: ItemState = {
  warehouses: [],
  items: [],
  pallets: [],
  boxes: [],
};

export const ItemReducer = (state = initialState, action: ItemActionTypes) => {
  switch (action.type) {
    case RECEIVE_WAREHOUSES:
      return {
        ...state,
        warehouses: action.warehouses,
      };
    case RECEIVE_ITEMS:
      return {
        ...state,
        items: action.items,
      };
    case RECEIVE_PALLETS:
      return {
        ...state,
        pallets: action.pallets,
      };
    case RECEIVE_BOXES:
      return {
        ...state,
        boxes: action.boxes,
      };
    default:
      return state;
  }
};
