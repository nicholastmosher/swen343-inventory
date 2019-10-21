import React, { Component } from "react";
import { connect } from "react-redux";
import { AppState } from "../reducers";
import {Pallet, Warehouse} from "../types/Interfaces";
import { ThunkDispatch } from "redux-thunk";
import { Link } from "react-router-dom";
import {fetchPallets} from "../actions/ItemActions";
import { RouteComponentProps } from "react-router-dom";

interface StateProps {
  pallets: Pallet[];
}

interface DispatchProps {
  fetchPallets: () => void;
}

interface URLParams {
  warehouseName: string;
}

type Props = StateProps & DispatchProps & RouteComponentProps<URLParams>;

class Pallets extends Component<Props, {}> {

  componentDidMount(): void {
    this.props.fetchPallets();
  }

  render() {
    const { pallets, match } = this.props;

    const palletComponents = pallets.map((pallet: Pallet) => (
      <div className="item-card col-sm-6">
        <a href={`/warehouses/${match.params.warehouseName}/pallet/${pallet.id}`}>
          <h2>Pallet: {pallet.id}</h2>
          <div className="details">{pallet.item_code}</div>
        </a>
      </div>
    ));

    return (
      <div className="content">
        <div className="nav">
          <div className="nav-header">
            <Link className="inventory-link" to="/">
              <h1 className="inventory-header">Inventory Management</h1>
            </Link>
            <Link className="header-link" to="/catalog">Catalog View</Link>
          </div>
        </div>

        <hr />

        <div className="row">
          {palletComponents}
        </div>
      </div>
    )
  }
}

const mapStateToProps = (state: AppState): StateProps => ({
  pallets: state.ItemReducer.pallets,
});

const mapDispatchToProps =
  (dispatch: ThunkDispatch<{}, {}, any>): DispatchProps => ({
    fetchPallets: () => dispatch(fetchPallets()),
  });

export default connect(mapStateToProps, mapDispatchToProps)(Pallets);
