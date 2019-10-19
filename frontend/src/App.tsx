import React from 'react';
import logo from './logo.svg';
import './App.css';
import Header from "./view/Header";
import Items from "./view/Items";

const App: React.FC = () => {
  return (
    <div className="App">
      <Header />
      <Items />
    </div>
  );
};

export default App;
