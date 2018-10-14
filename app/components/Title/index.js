import React from 'react';
import PropTypes from 'prop-types';
import cx from 'classnames';

import styles from './styles.scss';
import common from '../../styles.scss';

const Title = props => {
  return (
    <div className={cx(
      common.zeus,
      styles.title    
    )} >
      {props.children}
    </div>
  );
}

Title.propTypes = {
  children: PropTypes.node
};

export default Title;
