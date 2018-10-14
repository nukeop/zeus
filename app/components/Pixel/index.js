import React from 'react';
import PropTypes from 'prop-types';
import cx from 'classnames';

import styles from './styles.scss';
import common from '../../styles.scss';

const Pixel = props => {  
  return (
    <div className={cx(
      common.zeus,
      styles.pixel,
      {[`${styles.on}`]: props.on}     
    )} >
      {
        props.on
          ? <div className={styles.inner} />
          : null
      }
    </div>
  );
};

Pixel.propTypes = {
  on: PropTypes.bool
};

Pixel.defaultProps = {

};

export default Pixel;
