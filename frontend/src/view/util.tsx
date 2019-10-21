import React from "react";
import { Warehouse, Pallet } from "../types/Interfaces";

const itemDetails = (itemType: string, item: any) => {
  // elements to fill
  let details = null

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
      details = (
        <div className="details">
          <div><strong>Quantity:</strong> {pallet.item_quantity}</div>
        </div>
      )
      break
    // TODO: add pallets, boxes and units display formats here
    default:
      details = (<div>Unknown Container Type </div>)
  }

  let icon = (
    <div className="icon">
      <div className={`${itemType}-icon`}></div>
    </div>
  )

  return (
    <div className="item">{details}{icon}</div>
  )
}

/**
 * Create a pannel for navigation
 * @param itemType 
 * @param desc object with description details
 */
const navPanel = (itemType: string, desc: any) => {
  // TODO add nav elements here
  let nav = null
  /*
  let descElem = null;
  switch (itemType) {
    case "warehouse":
      descElem = (
        <div className="warehouse-panel">
          <h3>{desc.address}</h3>
          <div className="nav-tree">
            <div
          </div>
          <button>Reorder Rules</button>
          </div>
          )
          break
        default:
      }
    */
  return null
}


export default {
  itemDetails,
  navPanel
}