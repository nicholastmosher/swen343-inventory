import React, {Component, FormEvent} from 'react'
import util from './util';
import {ThunkDispatch} from "redux-thunk";
import {createItem} from "../actions/ItemActions";
import {Item} from "../types/Interfaces";
import {connect} from "react-redux";

interface State {
  name: string;
  cost: string;
  description: string;
  itemType: string;
}

const initialState = {
  name: "",
  cost: "",
  description: "",
  itemType: "",
};

interface DispatchProps {
  createItem: (item: Item) => void;
}

type Props = DispatchProps;

class AddItemForm extends Component<Props, State> {

  constructor(props: Props) {
    super(props);
    this.state = initialState;
  }

  handleName = (event: FormEvent<HTMLInputElement>) => {
    const name = event.currentTarget.value;
    this.setState({...this.state, name });
  };

  handleCost = (event: FormEvent<HTMLInputElement>) => {
    const cost = event.currentTarget.value;
    this.setState({ ...this.state, cost });
  };

  handleDescription = (event: FormEvent<HTMLTextAreaElement>) => {
    const description = event.currentTarget.value;
    this.setState({ ...this.state, description });
  };

  handleSelect = (event: FormEvent<HTMLSelectElement>) => {
    const itemType = event.currentTarget.value;
    this.setState({ ...this.state, itemType });
  };

  handleSubmit = () => {
    const item: Item = {
      code: `${this.state.itemType}:${this.state.name}`,
      cost: Number(this.state.cost),
      description: this.state.description,
    };
    this.props.createItem(item);
    this.setState(initialState);
  };

  render() {
    return (
      <div className="content">
        {util.inventoryHeader()}

        <hr/>

        <h3>Create a New Item</h3>
        <div className="item-form">

          <div className="flex-row">
            <span>Select Item Type</span>
            <select onChange={this.handleSelect}>
              <option value="part">Part</option>
              <option value="product">Product</option>
            </select>
          </div>

          <div className="flex-row">
            <span>Item Code</span>
            <input type="text" value={this.state.name} onChange={this.handleName}/>
          </div>

          <div className="flex-row">
            <span>Cost</span>
            <input type="text" value={this.state.cost} onChange={this.handleCost}/>
          </div>

          <div className="flex-row tall-field">
            <span>Description</span>
            <textarea name="itemDesc" value={this.state.description} onChange={this.handleDescription}/>
          </div>

          <div className="flex-all"/>
          <button className="full-button" onClick={this.handleSubmit}>create</button>
        </div>
      </div>
    );
  }
}

const mapDispatchToProps =
  (dispatch: ThunkDispatch<{}, {}, any>): DispatchProps => ({
    createItem: (item: Item) => dispatch(createItem(item))
  });

export default connect(state => state, mapDispatchToProps)(AddItemForm);
