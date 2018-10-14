import React from 'react';
import PropTypes from 'prop-types';
import cx from 'classnames';

import styles from './styles.scss';
import common from '../../styles.scss';

const ScreenDecoration = props => {
  return (
    <div className={cx(
      common.zeus,
      styles.screen_decoration
    )} >
      <div className={styles.title}>
        Super
      </div>
      {props.children}
    </div>
  );
}

ScreenDecoration.propTypes = {
  children: PropTypes.node
};

export default ScreenDecoration;
