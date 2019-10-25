import React from 'react'
import { Link } from 'react-router-dom'

interface Props {
  title: string;
  actionPath: string;
}

const CreateEntityCard: React.FC<Props> = ({ title, actionPath }) => (
  <Link className="col-sm-6 item-card create-item" to={actionPath}>
    <h2>{title}</h2>
    <div className="item">
      <div className="create-icon">+</div>
    </div>
  </Link>
);

export default CreateEntityCard;
