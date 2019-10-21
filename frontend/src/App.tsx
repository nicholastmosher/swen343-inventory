import React from 'react';
import logo from './logo.svg';
import './App.css';
import Header from "./view/Header";
import Items from "./view/Items";
import AddItem from "./view/AddItemForm";
import AddWarehouse from "./view/AddWarehouseForm";
import { Route, BrowserRouter as Router } from "react-router-dom";

const App: React.FC = () => {
  return (
    <div className="App">
      <Header />
      <Router>
        <div>
          <Route exact path="/" component={Items} />
          <Route exact path="/catalog" component={Items} />
          <Route exact path="/catalog/add" component={AddItem} />
          <Route exact path="/warehouse/add" component={AddWarehouse} />
          <Route exact path="/warehouses/:warehouseid" component={Items} />
          <Route exact path="/warehouses/:warehouseid/pallet/:palletid" component={Items} />
          <Route exact path="/warehouses/:warehouseid/pallet/:palletid/box/:boxid" component={Items} />
        </div>
      </Router>
    </div>
  );
};

export default App;
