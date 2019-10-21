import React from 'react';
import './App.css';
import Header from "./view/Header";
import Items from "./view/Items";
import AddItem from "./view/AddItemForm";
import AddWarehouse from "./view/AddWarehouseForm";
import { Route, BrowserRouter as Router } from "react-router-dom";
import ReorderRules from './view/ReorderRules';
import Warehouses from "./view/Warehouses";
import Pallets from "./view/Pallets";
import Boxes from "./view/Boxes";

const App: React.FC = () => {
  return (
    <div className="App">
      <Header />
      <Router>
        <div>
          <Route exact path="/" component={Warehouses} />
          <Route exact path="/catalog" component={Items} />
          <Route exact path="/catalog/add" component={AddItem} />
          <Route exact path="/warehouse/add" component={AddWarehouse} />
          <Route exact path="/warehouses/:warehouseName/reorder-rules" component={ReorderRules} />
          <Route exact path="/warehouses/:warehouseName" component={Pallets} />
          <Route exact path="/warehouses/:warehouseName/pallet/:palletid" component={Boxes} />
          <Route exact path="/warehouses/:warehouseName/pallet/:palletid/box/:boxid" component={Items} />
        </div>
      </Router>
    </div>
  );
};

export default App;
