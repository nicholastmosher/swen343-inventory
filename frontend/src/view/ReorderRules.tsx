import React from 'react'
import { Item } from '../types/Interfaces';
import util from './util';

const ReorderRules: React.FC = () => {
  let warehouseid = 1 // get from params
  // TODO: get the items from the backend
  let items: any[] = [{ code: 5 }, { code: 2 }]

  // TODO do this for all products
  let itemDivs: any = []
  for (let item of items) {
    itemDivs.push((
      <div>
        <hr />
        <form className="item-form" action={`/${item.code}`} method="get">
          <div className="flex-row">
            <span>Product Name: </span><strong>{item.code}</strong>
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
            <button className="full-button">update</button>
          </div>
        </form>
      </div>
    ))
  }

  return (
    <div className="content">
      {util.inventoryHeader()}

      <hr />

      <h3>Reorder Rules</h3>
      {itemDivs}
    </div>
  );
};

export default ReorderRules