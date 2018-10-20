import React from 'react';
import PropTypes from 'prop-types';
import cx from 'classnames';
import leftPad from 'left-pad';

import styles from './styles.scss';
import common from '../../styles.scss';

const Digit = props => {
  let {
    digit
  } = props;

  let d = 'digit_' + digit;
  
  return (
    <div className={cx(
      common.zeus,
      styles.digit     
    )}>
      <div className={cx(styles.segment, styles.a, d)} />
      <div className={cx(styles.segment, styles.b, d)} />
      <div className={cx(styles.segment, styles.c, d)} />
      <div className={cx(styles.segment, styles.d, d)} />
      <div className={cx(styles.segment, styles.e, d)} />
      <div className={cx(styles.segment, styles.f, d)} />
      <div className={cx(styles.segment, styles.g, d)}>
        <div className={styles.top} />
        <div className={styles.bottom} />
      </div>
    </div>
  );
};

const SevenSegment = props => {
  let {
    numberLeft,
    numberRight
  } = props;

  let digitsLeft = leftPad(numberLeft + '', 5, ' ').split('');
  let digitsRight = leftPad(numberRight + '', 5, ' ').split('');
  
  return (
    <div className={cx(
      common.zeus,
      styles.seven_segment
    )} >
      <Digit digit={digitsLeft[0]} />
      <Digit digit={digitsLeft[1]} />
      <Digit digit={digitsLeft[2]} />
      <Digit digit={digitsLeft[3]} />
      <Digit digit={digitsLeft[4]} />
      <div style={{width: '18px'}} />
      <Digit digit={digitsRight[0]} />
      <Digit digit={digitsRight[1]} />
      <Digit digit={digitsRight[2]} />
      <Digit digit={digitsRight[3]} />
      <Digit digit={digitsRight[4]} />
    </div>
  );
};

SevenSegment.propTypes = {
  numberLeft: PropTypes.number,
  numberRight: PropTypes.number
};

SevenSegment.defaultProps = {
  numberLeft: '',
  numberRight: ''
};

export default SevenSegment;
