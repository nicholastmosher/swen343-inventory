import React, { Component } from "react";
import { connect } from "react-redux";
import { AppState } from "../reducers";
import { Item } from "../types/Interfaces";
import { fetchItems } from "../actions/ItemActions";
import { ThunkDispatch } from "redux-thunk";
import { Link } from "react-router-dom";
import util from "./util";
import ButtonItem from "./AddButton";

interface StateProps {
  items: Item[];
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
    const { items, fetchItems } = this.props;

    // TODO: get this from the url (or state, or router)
    let itemType = "warehouse"

    // generate the item cards
    const itemDivs = []
    for (let itemId in items) {
      //let item = items[itemId] // TODO: swap this for real id

      const name = (<h2>{itemType} #{itemId}</h2>)
      const detailsDiv = (<div className="details">details</div>)
      itemDivs.push(<div className="item-card col-sm-6">{name}{detailsDiv}</div>)
    }

    // add the optional 'add' element
    if (itemType == "warehouse") {
      itemDivs.push(ButtonItem("Add Warehouse", "/warehouse/add"))
    } else if (itemType == "catalog") {
      itemDivs.push(ButtonItem("Add Item", "/catalog/add"))
    }


    return (
      <div className="content">
        <div className="nav">
          <div className="nav-header">
            <h1 className="inventory-header">Inventory Management</h1>
            <Link className="header-link" to="/catalog">Catalog View</Link>
          </div>
          {util.navPanel}
        </div>

        <hr />

        <div className="row">{itemDivs}</div>
      </div>
    )
  }
}

const mapStateToProps = (state: AppState): StateProps => ({
  items: state.ItemReducer.items,
});

const mapDispatchToProps =
  (dispatch: ThunkDispatch<{}, {}, any>): DispatchProps => ({
    fetchItems: () => dispatch(fetchItems())
  });

export default connect(mapStateToProps, mapDispatchToProps)(Items);
