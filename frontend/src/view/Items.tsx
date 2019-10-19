import React from "react";
import { connect } from "react-redux";
import { AppState } from "../reducers";
import { Item } from "../types/ItemActionTypes";
import { fetchItems } from "../actions/ItemActions";
import { ThunkDispatch } from "redux-thunk";

interface StateProps {
  items: Item[];
}

interface DispatchProps {
  fetchItems: () => void;
}

type Props = StateProps & DispatchProps;

const Items: React.FC<Props> = ({ items, fetchItems }) => {
  // temporary define for items
  let fakeItems: Item[] = [{
    code: "5",
    cost: 500
  }]
  items = fakeItems;

  return (
    <div>
      <ul>
        {items.map(item => <li>{JSON.stringify(item)}</li>)}
      </ul>
      <button onClick={() => fetchItems()}>Get Items</button>
    </div>)
};

const mapStateToProps = (state: AppState): StateProps => ({
  items: state.ItemReducer.items,
});

const mapDispatchToProps =
  (dispatch: ThunkDispatch<{}, {}, any>): DispatchProps => ({
    fetchItems: () => dispatch(fetchItems())
  });

export default connect(mapStateToProps, mapDispatchToProps)(Items);
