import React from 'react'
import util from './util';

const AddItem: React.FC = () => {
  return (
    <div className="content">
      {util.inventoryHeader()}

      <hr />

      <h3>Create a New Item</h3>
      <form className="item-form" action="/catalog" method="get">

        <div className="flex-row">
          <span>Select Item Type</span>
          <select>
            <option value="part">Part</option>
            <option value="product">Product</option>
          </select>
        </div>
        <div className="flex-row">
          <span>Name</span>
          <input type="text" name="itemName"></input>
        </div>
        <div className="flex-row tall-field">

          <span>Description</span>
          <textarea name="itemDesc"></textarea>
        </div>
        <div className="flex-row">
          <span>Tags</span>
          <select multiple>
            <option value="F">Fashions</option>
            <option value="C">Comforts</option>
            <option value="H">Healthies</option>
            <option value="A">Actives</option>
            <option value="B">Biofeedback</option>
          </select>
        </div>
        <div className="flex-row">
          <span>Parts</span>
          <select multiple>
            <option value="partA">Part A</option>
            <option value="partB">Part B</option>
            <option value="partC">Part C</option>
          </select>
        </div>

        <div className="flex-all"></div>
        <button className="full-button">create</button>
      </form>
    </div >
  );
};

export default AddItem
