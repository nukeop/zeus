import React from 'react';
import PropTypes from 'prop-types';
import cx from 'classnames';

import styles from './styles.scss';
import common from '../../styles.scss';

const Menu = props => {
  return (
    <div className={cx(
      common.zeus,
      styles.menu    
    )} >
      { props.children }
    </div>
  );
};

Menu.propTypes = {
  children: PropTypes.node
};

export default Menu;
