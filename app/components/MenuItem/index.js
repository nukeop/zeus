import React from 'react';
import PropTypes from 'prop-types';
import cx from 'classnames';

import styles from './styles.scss';
import common from '../../styles.scss';

const MenuItem = props => {
  return (
    <div
      className={cx(
        common.zeus,
        styles.menu_item   
      )}
      onClick={props.onClick}
    >
      { props.children }
    </div>
  );
}

MenuItem.propTypes = {
  children: PropTypes.node,
  onClick: PropTypes.func
};

export default MenuItem;
