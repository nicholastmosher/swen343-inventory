import React, { Component } from "react";
import { connect } from "react-redux";
import { AppState } from "../reducers";
import { Item } from "../types/Interfaces";
import { ThunkDispatch } from "redux-thunk";
import ButtonItem from "./AddButton";
import { fetchItems } from "../actions/ItemActions";

interface StateProps {
  items: Item[],
}

interface DispatchProps {
  fetchItems: () => void;
}

type Props = StateProps & DispatchProps;

class Items extends Component<Props, {}> {

  componentDidMount(): void {
    this.props.fetchItems();
  }

  render() {
    const { items } = this.props;

    const itemComponents = items.map((item: Item) => (
      <div className="item-card col-sm-6">
        <h2>{item.code}</h2>
        <div>${Number(item.cost / 100).toFixed(2)}</div>
        <div>{item.description}</div>
      </div>
    ));

    return (
        <>
          {itemComponents}
          {ButtonItem("Add Item", "/catalog/add")}
        </>
    )
  }
}

const mapStateToProps = (state: AppState): StateProps => ({
  items: state.ItemReducer.items,
});

const mapDispatchToProps =
  (dispatch: ThunkDispatch<{}, {}, any>): DispatchProps => ({
    fetchItems: () => dispatch(fetchItems()),
  });

export default connect(mapStateToProps, mapDispatchToProps)(Items);
