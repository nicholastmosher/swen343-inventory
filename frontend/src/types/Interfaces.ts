export interface Item {
  code: string,
  cost: number,
  description?: string,
}

export interface Warehouse {
  name: string,
  address: string,
}

export interface Pallet {
  pallet_id: number,
  item_quantity: number
}

export interface Box {
  item_code: string,
  warehouse_name: string
}
