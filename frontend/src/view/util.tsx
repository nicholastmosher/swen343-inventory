import React from "react";
import { Warehouse, Pallet, Box, Item } from "../types/Interfaces";
import { Link } from "react-router-dom";

const itemDetails = (itemType: string, item: any) => {
  // elements to fill
  let details = null

  switch (itemType) {
    case "warehouse":
      let warehouse: Warehouse = item;
      details = (
        <div className="details">
          <div><strong>Name:</strong> {warehouse.name}</div>
          <div><strong>Address:</strong> {warehouse.address}</div>
        </div>
      )
      break;
    case "pallet":
      let pallet: Pallet = item;
      details = (
        <div className="details">
          <div><strong>ID:</strong> {pallet.pallet_id}</div>
          <div><strong>Quantity:</strong> {pallet.item_quantity}</div>
        </div>
      )
      break
    // TODO: add pallets, boxes and units display formats here
    case "box":
      let box: Box = item;
      details = (
        <div className="details">
          <div><strong>ID:</strong> {box.item_code}</div>
          <div><strong>Name:</strong> {box.warehouse_name}</div>
        </div>
      )
      break
    case "unit":
      let unit: Item = item;
      details = (
        <div className="details">
          <div><strong>ID:</strong> {unit.code}</div>
          <div><strong>Cost:</strong> {unit.cost}</div>
          <div><strong>Description:</strong> {unit.description}</div>
        </div>
      )
      break

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
const navPanel = (itemType: string) => {
  let breadcrumbs = null
  let descElem = null
  switch (itemType) {
    case "warehouse":
      // TODO: add the address here when once the backend sends it over
      descElem = (
        <div className="warehouse-panel">
          <div className="nav-tree"></div>
          <Link to="/reorder-rules">Reorder Rules</Link>
        </div>
      )
      break
    default:
  }
  return descElem
}


export default {
  itemDetails,
  navPanel
}