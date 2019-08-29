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
      <div className={cx(
        common.zeus,
        sevenSegmentStyles.seven_segment,
        styles.bottom_seven_segment
      )} >
        <span className={styles.speed}>
          Speed
        </span>
        {
          digitsLeft.map((digit, i) => {
            return (<Digit key={i} digit={digit}/>);
          })
        }
      </div>
      <div className={cx(
        common.zeus,
        sevenSegmentStyles.seven_segment,
        styles.bottom_seven_segment
      )} >
        <span className={styles.level}>
          Level
        </span>
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
