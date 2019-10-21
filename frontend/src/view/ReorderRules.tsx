import React from 'react'
import { Item } from '../types/Interfaces';

const ReorderRules: React.FC = () => {
  let warehouseid = 1 // get from params
  // TODO: get the items from the backend
  let items: Item[] = []

  // TODO do this for all products
  let itemDivs: any = [{ code: 5 }]
  for (let item of items) {
    itemDivs.push((
      <form className="item-form" action={`/${item.code}`} method="get">
        <div className="flex-row">
          <span>{item.code}</span>
          <input type="text" name="product"></input>
        </div>
        <div className="flex-row tall-field">
          <span>When less than:</span>
          <input type="text" name="min"></input>
        </div>
        <div className="flex-row tall-field">
          <span>Order this many:</span>
          <input type="text" name="product"></input>
        </div>
        <div className="flex-all">
          <button className="full-button">create</button>
        </div>
      </form>
    ))
  }

  return (
    <div className="content">
      <h1 className="inventory-header">Inventory Management</h1>

      <hr />

      <h3>Reorder Rules</h3>
      {itemDivs}
    </div>
  );
};

export default ReorderRules