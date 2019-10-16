import React from "react";
import { Warehouse, Pallet, Box, Item } from "../types/Interfaces";
import { Link } from "react-router-dom";

const getIcon = (itemType: string) => {
  return (
    <div className={`${itemType}-icon`}></div>
  )
}

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
          <div><strong>ID:</strong> {pallet.id}</div>
          <div><strong>Quantity:</strong> {pallet.item_code}</div>
        </div>
      )
      break
    // TODO: add pallets, boxes and units display formats here
    case "box":
      let box: Box = item;
      details = (
        <div className="details">
          {/*<div><strong>ID:</strong> {box.item_code}</div>*/}
          {/*<div><strong>Name:</strong> {box.warehouse_name}</div>*/}
          {/*<div><strong>Count:</strong> {box.warehouse_name}</div>*/}
        </div>
      )
      break

    default:
      details = (<div>Unknown Container Type </div>)
  }

  let icon = (
    <div className="icon">
      getIcon(itemType)
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
const navPanel = (itemType: string, ids: any) => {
  let breadcrumbs: any[] = []
  let elem: any = null

  if (itemType == "warehouse") {
    elem = (
      <div className="warehouse-panel">
        <div className="nav-tree"></div>
        <Link className="reorder-rules-link" to={`/warehouses/${ids.warehouseid}/reorder-rules`}>Reorder Rules</Link>
      </div>
    )
  }

  switch (itemType) {
    case "box":
      breadcrumbs.push(
        <div className="location">
          <div className="nav-icon">
            {getIcon("box")}
          </div>
          <div className="pathId">
            #{ids.boxid}
          </div>
        </div>
      )
    case "pallet":
      breadcrumbs.push(
        <div className="location">
          <div className="nav-icon">
            {getIcon("pallet")}
          </div>
          <div className="pathId">
            #{ids.palletid}
          </div>
        </div>
      )
    case "warehouse":
      breadcrumbs.push(
        <div className="location">
          <div className="nav-icon">
            {getIcon("warehouse")}
          </div>
          <div className="pathId">
            #{ids.warehouseid}
          </div>
        </div>
      )
    default:
  }
  return (
    <div className="nav-tree">{elem}{breadcrumbs}</div>
  )
}

const inventoryHeader = () => {
  return (<Link className="inventory-link" to="/">
    <h1 className="inventory-header">Inventory Management</h1>
  </Link>)
}

export default {
  itemDetails,
  navPanel,
  inventoryHeader
}