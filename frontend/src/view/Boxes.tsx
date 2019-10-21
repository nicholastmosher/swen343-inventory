import React, { Component } from "react";
import { connect } from "react-redux";
import { AppState } from "../reducers";
import {Box, Pallet, Warehouse} from "../types/Interfaces";
import { ThunkDispatch } from "redux-thunk";
import { Link } from "react-router-dom";
import {fetchBoxes, fetchPallets} from "../actions/ItemActions";
import { RouteComponentProps } from "react-router-dom";

interface StateProps {
  boxes: Box[];
}

interface DispatchProps {
  fetchBoxes: () => void;
}

interface URLParams {
  warehouseName: string;
  palletId: string;
}

type Props = StateProps & DispatchProps & RouteComponentProps<URLParams>;

class Boxes extends Component<Props, {}> {

  componentDidMount(): void {
    this.props.fetchBoxes();
  }

  render() {
    const { boxes, match } = this.props;

    const boxComponents = boxes.map((box: Box) => (
      <div className="item-card col-sm-6">
        <a href={`/warehouses/${match.params.warehouseName}/pallet/${match.params.palletId}/`}>
          <h2>Box: {box.id}</h2>
          <div className="details">{box.item_quantity}</div>
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
          {boxComponents}
        </div>
      </div>
    )
  }
}

const mapStateToProps = (state: AppState): StateProps => ({
  boxes: state.ItemReducer.boxes,
});

const mapDispatchToProps =
  (dispatch: ThunkDispatch<{}, {}, any>): DispatchProps => ({
    fetchBoxes: () => dispatch(fetchBoxes()),
  });

export default connect(mapStateToProps, mapDispatchToProps)(Boxes);
