import React from "react";
import {Link, RouteComponentProps} from "react-router-dom";

type Props = RouteComponentProps<{}>;

const ContentHeader: React.FC<Props> = (props: Props) => {

  const headerLink = (props.location.pathname.includes("catalog")) ?
    <Link className="header-link" to="/">Inventory view</Link> :
    <Link className="header-link" to="/catalog">Catalog view</Link>;

  return (
    <div className="nav">
      <div className="nav-header">
        <Link className="inventory-link" to="/">
          <h1 className="inventory-header">Inventory Management</h1>
        </Link>
        {headerLink}
      </div>
    </div>
  );
};

export default ContentHeader;
