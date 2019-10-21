import { combineReducers } from "redux";
import { ItemReducer } from "./ItemReducer";

export const rootReducer = combineReducers({
  ItemReducer,
});

export type AppState = ReturnType<typeof rootReducer>;
