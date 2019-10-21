import React, { Component } from "react";
import { connect } from "react-redux";
import { AppState } from "../reducers";
import { Warehouse } from "../types/Interfaces";
import { ThunkDispatch } from "redux-thunk";
import { Link } from "react-router-dom";
import ButtonItem from "./AddButton";
import {fetchWarehouses} from "../actions/ItemActions";

interface StateProps {
  warehouses: Warehouse[],
}

interface DispatchProps {
  fetchWarehouses: () => void;
}

type Props = StateProps & DispatchProps;

class Items extends Component<Props, {}> {

  componentDidMount(): void {
    this.props.fetchWarehouses();
  }

  render() {
    const { warehouses } = this.props;

    const warehouseComponents = warehouses.map((warehouse: Warehouse) => (
      <div className="item-card col-sm-6">
        <a href={`/warehouses/${warehouse.name}`}>
          <h2>Warehouse: {warehouse.name}</h2>
          <div className="details">{warehouse.address}</div>
        </a>
      </div>
    ));

    return (
      <>
          {warehouseComponents}
          {ButtonItem("Add Warehouse", "/warehouse/add")}
      </>
    )
  }
}

const mapStateToProps = (state: AppState): StateProps => ({
  warehouses: state.ItemReducer.warehouses,
});

const mapDispatchToProps =
  (dispatch: ThunkDispatch<{}, {}, any>): DispatchProps => ({
    fetchWarehouses: () => dispatch(fetchWarehouses()),
  });

export default connect(mapStateToProps, mapDispatchToProps)(Items);
