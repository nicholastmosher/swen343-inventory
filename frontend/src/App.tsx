import React from 'react';
import logo from './logo.svg';
import './App.css';
import Header from "./view/Header";
import Items from "./view/Items";
import { Route, Link, BrowserRouter as Router } from "react-router-dom";

const App: React.FC = () => {
  return (
    <div className="App">
      <Header />
      <Router>
        <div>
          <Route path="/" component={Items} />
        </div>
      </Router>
    </div>
  );
};

export default App;
