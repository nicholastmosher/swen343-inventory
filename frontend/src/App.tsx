import React from 'react';
import './App.css';
import Header from "./view/Header";
import Items from "./view/Items";
import AddItem from "./view/AddItemForm";
import AddWarehouse from "./view/AddWarehouseForm";
import {BrowserRouter as Router, Route, Switch} from "react-router-dom";
import ReorderRules from './view/ReorderRules';
import Warehouses from "./view/Warehouses";
import Pallets from "./view/Pallets";
import Boxes from "./view/Boxes";
import Catalog from "./view/Catalog";
import ContentHeader from "./view/ContentHeader";

const App: React.FC = () => {
  return (
    <div className="App">
      <Header />

      <Router>
        <div className="content">
          <Switch>
            <Route exact path="/" component={ContentHeader} />
            <Route exact path="/catalog" component={ContentHeader} />
          </Switch>
          <hr />

          <div className="row">
              <Switch>
                <Route exact path="/" component={Warehouses} />
                <Route exact path="/catalog" component={Catalog} />
                <Route exact path="/catalog/add" component={AddItem} />
                <Route exact path="/warehouse/add" component={AddWarehouse} />
                <Route exact path="/warehouses/:warehouseName/reorder-rules" component={ReorderRules} />
                <Route exact path="/warehouses/:warehouseName" component={Pallets} />
                <Route exact path="/warehouses/:warehouseName/pallet/:palletId" component={Boxes} />
                <Route exact path="/warehouses/:warehouseName/pallet/:palletId/box/:boxId" component={Items} />
              </Switch>
          </div>
        </div>
      </Router>
    </div>
  );
};

export default App;
