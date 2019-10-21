export interface Warehouse {
  name: string,
  address: string,
}

export interface Item {
  code: string,
  cost: number,
  description?: string,
}

export interface Pallet {
  id: number;
  item_code: number;
  warehouse_name: string;
}

export interface Box {
  id: number;
  pallet_id: string;
  item_quantity: string;
}
