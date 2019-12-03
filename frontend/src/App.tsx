import React from 'react';
import './App.css';
import { HashRouter as Router, Route, Switch } from "react-router-dom";
import Header from "./view/components/Header";
import ContentHeader from "./view/components/ContentHeader";
import AddItemForm from "./view/AddItemForm";
import AddWarehouseForm from "./view/AddWarehouseForm";
import ReorderRules from './view/ReorderRules';
import Warehouses from "./view/Warehouses";
import Pallets from "./view/Pallets";
import Boxes from "./view/Boxes";
import Catalog from "./view/Catalog";

const App: React.FC = () => {
  //<Route exact path="/catalog" component={Catalog} />
  return (
    <div className="App">
      <Header />

      <Router>
        <div className="content">
          <Switch>
            <Route path="/" component={ContentHeader} />
            <Route path="/catalog" component={ContentHeader} />
          </Switch>
          <hr />

          <div className="row">
            <Switch>
              <Route exact path="/" component={Warehouses} />
              <Route exact path="/catalog/add" component={AddItemForm} />
              <Route exact path="/warehouse/add" component={AddWarehouseForm} />
              <Route exact path="/warehouse/:warehouseName" component={Pallets} />
              <Route exact path="/warehouse/:warehouseName/reorder-rules" component={ReorderRules} />
              <Route exact path="/warehouse/:warehouseName/pallet/:palletId" component={Boxes} />
            </Switch>
          </div>
        </div>
      </Router>
    </div>
  );
};

export default App;
