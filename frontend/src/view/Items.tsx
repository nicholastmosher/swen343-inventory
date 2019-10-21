import React, { Component } from "react";
import { connect } from "react-redux";
import { AppState } from "../reducers";
import { Box, Item, Pallet, Warehouse } from "../types/Interfaces";
import { fetchBoxes, fetchItems, fetchPallets, fetchWarehouses } from "../actions/ItemActions";
import { ThunkDispatch } from "redux-thunk";
import { Link, useLocation, useParams } from "react-router-dom";
import util from "./util";
import ButtonItem from "./AddButton";

interface StateProps {
  warehouses: Warehouse[],
  items: Item[];
  pallets: Pallet[],
  boxes: Box[],
}

interface DispatchProps {
  fetchWarehouses: () => void;
  fetchItems: () => void;
  fetchPallets: () => void;
  fetchBoxes: () => void;
}

type Props = StateProps & DispatchProps;

class Items extends Component<Props, {}> {

  componentDidMount(): void {
    this.props.fetchWarehouses();
    this.props.fetchItems();
    this.props.fetchPallets();
    this.props.fetchBoxes();
  }

  render() {
    const { warehouses, pallets, boxes, items } = this.props;
    let units = warehouses || pallets || boxes || items

    // figure oute what page is currently active
    //const loc = useLocation()
    //const path = loc.pathname
    const page: any = "not catalog" //path.split("/").pop()
    const { warehouseid, palletid, boxid }: any = {} // useParams()
    const ids = { warehouseid, palletid, boxid }

    // TEMP: you can manually switch this out to get diffent views
    // TODO: have itemType automatically get set 
    let unitType = "warehouse"
    if (page == "catalog") {
      unitType = "catalog"
    } else {
      // get the params

      if (boxid) {
        unitType = "unit"
      } else if (palletid) {
        unitType = "box"
      } else if (warehouseid) {
        unitType = "pallet"
      } else {
        unitType = "warehouse"
      }
    }

    // TODO: get this from the url (or state, or router)
    // This is the type of the items being displayed, not the current container
    // this needs to be one of:
    // warehouse, pallet, box, unit

    // generate the unit cards
    const unitDivs = []
    for (let unitId in units) { // TODO: => (item of items)
      //let item = items[itemId] // TODO: swap this for real id
      const name = (<h2>{unitType} #{unitId}</h2>)
      const detailsDiv = (<div className="details">details</div>)
      unitDivs.push(<div className="item-card col-sm-6">{name}{detailsDiv}</div>)
    }

    // add the optional 'add' element
    let headerLink = null
    if (unitType == "warehouse") {
      unitDivs.push(ButtonItem("Add Warehouse", "/warehouse/add"))
      headerLink = (<Link className="header-link" to="/catalog">Catalog View</Link>)
    } else if (unitType == "catalog") {
      unitDivs.push(ButtonItem("Add Item", "/catalog/add"))
      headerLink = (<Link className="header-link" to="/">Inventory View</Link>)
    }
    //  {util.navPanel(unitType)}
    return (
      <div className="content">
        <div className="nav">
          <div className="nav-header">
            {/*{util.inventoryHeader()}*/}
            {headerLink}
          </div>
          {util.navPanel(unitType, ids)}
        </div>

        <hr />

        <div className="row">{unitDivs}</div>
      </div>
    )
  }
}

const mapStateToProps = (state: AppState): StateProps => ({
  warehouses: state.ItemReducer.warehouses,
  items: state.ItemReducer.items,
  pallets: state.ItemReducer.pallets,
  boxes: state.ItemReducer.boxes,
});

const mapDispatchToProps =
  (dispatch: ThunkDispatch<{}, {}, any>): DispatchProps => ({
    fetchWarehouses: () => dispatch(fetchWarehouses()),
    fetchItems: () => dispatch(fetchItems()),
    fetchPallets: () => dispatch(fetchPallets()),
    fetchBoxes: () => dispatch(fetchBoxes()),
  });

export default connect(mapStateToProps, mapDispatchToProps)(Items);
