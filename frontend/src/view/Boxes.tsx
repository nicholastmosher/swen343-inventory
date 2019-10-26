import React, {Component} from "react";
import {connect} from "react-redux";
import {AppState} from "../reducers";
import {Box} from "../types/Interfaces";
import {ThunkDispatch} from "redux-thunk";
import {RouteComponentProps} from "react-router-dom";
import {fetchBoxes} from "../actions/ItemActions";

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

    return boxes
      .filter(box => box.pallet_id.toString() === match.params.palletId)
      .map((box: Box) => (
        <div className="item-card col-sm-6">
          <h2>Box: {box.id}</h2>
          <div>Pallet: {box.pallet_id}</div>
          <div className="details">{box.item_quantity}</div>
        </div>
      ));
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
