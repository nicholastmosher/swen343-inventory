import React, {Component} from "react";
import {connect} from "react-redux";
import {AppState} from "../reducers";
import {Pallet} from "../types/Interfaces";
import {ThunkDispatch} from "redux-thunk";
import {Link, RouteComponentProps} from "react-router-dom";
import {fetchPallets} from "../actions/ItemActions";

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

    return pallets
      .filter(pallet => pallet.warehouse_name === match.params.warehouseName)
      .map(pallet => (
        <div className="item-card col-sm-6">
          <Link to={`/warehouse/${match.params.warehouseName}/pallet/${pallet.id}`}>
            <h2>Pallet: {pallet.id}</h2>
            <div className="details">{pallet.item_code}</div>
          </Link>
        </div>
      ));
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
