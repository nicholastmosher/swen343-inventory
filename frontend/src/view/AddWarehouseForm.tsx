import React, {Component, FormEvent} from 'react'
import {ThunkDispatch} from "redux-thunk";
import {Warehouse} from "../types/Interfaces";
import {insertWarehouse} from "../actions/ItemActions";
import {connect} from "react-redux";
import util from './util';

interface State {
  name: string,
  address: string,
}

interface DispatchProps {
  insertWarehouse: (warehouse: Warehouse) => void;
}

type Props = DispatchProps;

class AddWarehouse extends Component<Props, State> {

  constructor(props: Props) {
    super(props);

    this.state = {
      name: "",
      address: "",
    };
  }

  handleNameChange = (event: FormEvent<HTMLInputElement>) => {
    this.setState({
      ...this.state,
      name: event.currentTarget.value,
    });
  };

  handleDescriptionChange = (event: FormEvent<HTMLInputElement>) => {
    this.setState({
      ...this.state,
      address: event.currentTarget.value,
    })
  };

  handleSubmit = () => {
    const warehouse = {
      name: this.state.name,
      address: this.state.address,
    };
    this.props.insertWarehouse(warehouse);
    this.setState({ name: "", address: "" });
  };

  render() {
    return (
      <div className="content">
        {util.inventoryHeader()}

        <hr />

        <h3>Create Warehouse</h3>
        <div className="item-form">
          <div className="flex-row">
            <span>Name</span>
            <input type="text" onChange={this.handleNameChange} value={this.state.name} />
          </div>
          <div className="flex-row tall-field">
            <span>Address</span>
            <input type="text" onChange={this.handleDescriptionChange} value={this.state.address} />
          </div>
          <div className="flex-all">
            <button className="full-button" onClick={this.handleSubmit}>Create</button>
          </div>
        </div>
      </div>
    );
  }
}

const mapDispatchToProps =
  (dispatch: ThunkDispatch<{}, {}, any>): DispatchProps => ({
    insertWarehouse: (warehouse: Warehouse) => dispatch(insertWarehouse(warehouse))
  });

export default connect(state => state, mapDispatchToProps)(AddWarehouse);
