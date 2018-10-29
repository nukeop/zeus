import React from 'react';
import PropTypes from 'prop-types';
import cx from 'classnames';

import { Digit } from '../SevenSegment';
import styles from './styles.scss';
import common from '../../styles.scss';
import sevenSegmentStyles from '../SevenSegment/styles.scss';


const BottomSevenSegment = props => {
  let {
    digitsLeft,
    digitsRight
  } = props;
  
  return (
    <div className={cx(
      common.zeus,
      styles.bottom_seven_segment_container
    )}>
      <label>SPEED</label>
      <div className={cx(
        common.zeus,
        sevenSegmentStyles.seven_segment
      )} >
        {
          digitsLeft.map((digit, i) => {
            return (<Digit key={i} digit={digit}/>);
          })
        }
      </div>
      <label>LEVEL</label>
      <div className={cx(
        common.zeus,
        sevenSegmentStyles.seven_segment
      )} >
        {
          digitsRight.map((digit, i) => {
            return (<Digit key={i} digit={digit}/>);
          })
        }
      </div>
    </div>
  );
};

BottomSevenSegment.propTypes = {
  digitsLeft: PropTypes.array,
  digitsRight: PropTypes.array
};

BottomSevenSegment.defaultProps = {
  digitsLeft: [],
  digitsRight: []
};

export default BottomSevenSegment;
