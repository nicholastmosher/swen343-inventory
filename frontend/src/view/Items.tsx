import React from "react";
import { connect } from "react-redux";
import { AppState } from "../reducers";
import { Item } from "../types/Interfaces";
import { fetchItems } from "../actions/ItemActions";
import { ThunkDispatch } from "redux-thunk";
import { Link } from "react-router-dom";
import util from "./util";

interface StateProps {
  items: Item[];
}

interface DispatchProps {
  fetchItems: () => void;
}

type Props = StateProps & DispatchProps;

const Items: React.FC<Props> = ({ items, fetchItems }) => {
  // get this from the url (or state, or router)
  let itemType = "warehouse"

  // temporary define for items
  let fakeItems: any[] = [{
    address: "i live here",
  }]
  items = fakeItems;

  // create (and fill) item cards
  const itemDivs = []
  for (let itemId in items) {
    let item = items[itemId] // TODO: swap this for real id
    const name = (<h2 className="item-card-header">{itemType} #{itemId}</h2>)
    const itemDiv = util.itemDetails(itemType, item)
    itemDivs.push(<div className="col-sm-6 item-card">{name}{itemDiv}</div>)
  }

  /* <button onClick={() => fetchItems()}>Get Items</button> */

  return (
    <div className="container pl-xs-0 pr-xs-0 pl-md-3 pr-md-3 content">
      <div className="nav">
        <div className="nav-header">
          <h1 className="inventory-header">Inventory Management</h1>
          <Link className="header-link" to="/catalog">Catalog View</Link>
        </div>
      </div>

      <hr />

      <div className="row">{itemDivs}</div>
    </div>
  )
};

const mapStateToProps = (state: AppState): StateProps => ({
  items: state.ItemReducer.items,
});

const mapDispatchToProps =
  (dispatch: ThunkDispatch<{}, {}, any>): DispatchProps => ({
    fetchItems: () => dispatch(fetchItems())
  });

export default connect(mapStateToProps, mapDispatchToProps)(Items);
