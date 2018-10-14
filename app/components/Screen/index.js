import React from 'react';
import PropTypes from 'prop-types';
import cx from 'classnames';

import Pixel from '../Pixel';

import styles from './styles.scss';
import common from '../../styles.scss';

const Screen = props => {
  return (
    <div className={cx(
      common.zeus,
      styles.screen
    )}
    >
      {
        props.screenData.map((pixel, i) => {
          return (
            <Pixel key={i} on={pixel} />
          );
        })
      }
    </div>
  );
};

Screen.propTypes = {
  screenData: PropTypes.array
};

Screen.defaultProps = {
  screenData: []
};

export default Screen;
