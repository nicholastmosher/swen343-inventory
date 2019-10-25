import React, { Component } from "react";
import { connect } from "react-redux";
import { AppState } from "../reducers";
import { Item } from "../types/Interfaces";
import { ThunkDispatch } from "redux-thunk";
import CreateEntityCard from "./components/CreateEntityCard";
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
      <ItemCard item={item} />
    ));

    return (
        <>
          {itemComponents}
          <CreateEntityCard title="Add Item" actionPath="/catalog/add"/>
        </>
    )
  }
}

interface ItemCardProps {
  item: Item;
}

const ItemCard: React.FC<ItemCardProps> = ({ item }) => {
  const ITEM_REGEX = /^([^:]*?):(.*?)$/;
  const matches = ITEM_REGEX.exec(item.code)!;
  const item_type = matches[1];
  const item_name = matches[2];

  return (
    <div className="item-card col-sm-6">
      <h2>{item_name} <span className="badge badge-secondary">{item_type}</span></h2>
      <div>${Number(item.cost / 100).toFixed(2)}</div>
      <div>{item.description}</div>
    </div>
  );
};

const mapStateToProps = (state: AppState): StateProps => ({
  items: state.ItemReducer.items,
});

const mapDispatchToProps =
  (dispatch: ThunkDispatch<{}, {}, any>): DispatchProps => ({
    fetchItems: () => dispatch(fetchItems()),
  });

export default connect(mapStateToProps, mapDispatchToProps)(Items);
