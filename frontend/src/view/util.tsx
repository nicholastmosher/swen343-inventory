import React from "react";
import { Warehouse, Pallet } from "../types/Interfaces";

const itemDetails = (itemType: string, item: any) => {

  // elements to fill
  let details = null
  let nav = null

  switch (itemType) {
    case "warehouse":
      let warehouse: Warehouse = item;
      details = (
        <div className="details">{warehouse.address}</div>
      )
      break;
    case "pallet":
      let pallet: Pallet = item;

      // get the address from the current warehouse
      // -> get request for warehouse from the backend      
      let address = "19, bocker street, rochester ny"

      // create display elements
      nav = (
        <div className="warehouse-panel" >
          <h3>{address}</h3>
        </div>
      )
      details = (
        <div className="details">
          <div>Quantity: {pallet.item_quantity}</div>
        </div>
      )
      break
    // TODO: add pallets, boxes and items display formats here
    default:
      details = (<div>Unknown Container Type </div>)
  }

  let icon = (
    <div className="icon">
      <div className={`${itemType}-icon`}></div>
    </div>
  )

  return (<div className="item">{details}{icon}</div>)
}

export default {
  itemDetails
}