export interface Item {
  code: String,
  cost: number,
  description?: String,
}

export interface Warehouse {
  name: String,
  address: String,
}

export interface Pallet {
  pallet_id: number,
  item_quantity: number
}

export interface Box {
  item_code: string,
  warehouse_name: string
}