import React from 'react'

const AddWarehouse: React.FC = () => {
  return (
    <div className="content">
      <h1 className="inventory-header">Inventory Management</h1>

      <hr />

      <h3>Create Warehouse</h3>
      <form className="item-form" action="/" method="get">
        <div className="flex-row">
          <span>Name</span>
          <input type="text" name="itemName"></input>
        </div>
        <div className="flex-row tall-field">
          <span>Description</span>
          <textarea name="itemDesc"></textarea>
        </div>
        <div className="flex-all">
          <button className="full-button">create</button>
        </div>
      </form>
    </div>
  );
};

export default AddWarehouse