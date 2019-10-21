import React from 'react'
import { Link } from 'react-router-dom'

const ButtonItem = (title: string, actionPath: string) => {
  return (
    <Link className="col-sm-6 item-card create-item" to={actionPath}>
      <h2>{title}</h2>
      <div className="item">
        <div className="create-icon">+</div>
      </div>
    </Link>
  )
}

export default ButtonItem