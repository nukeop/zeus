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

  
  return (
    <div className={cx(
      common.zeus,
      styles.digit     
    )}>
      <div className={cx(styles.segment, styles.a, {'on': digit[0]})} />
      <div className={cx(styles.segment, styles.b, {'on': digit[1]})} />
      <div className={cx(styles.segment, styles.c, {'on': digit[2]})} />
      <div className={cx(styles.segment, styles.d, {'on': digit[3]})} />
      <div className={cx(styles.segment, styles.e, {'on': digit[4]})} />
      <div className={cx(styles.segment, styles.f, {'on': digit[5]})} />
      <div className={cx(styles.segment, styles.g, {'on': digit[6]})}>
        <div className={styles.top} />
        <div className={styles.bottom} />
      </div>
    </div>
  );
};

const SevenSegment = props => {
  let {
    score,
    hiScore
  } = props;
  
  return (
    <div className={cx(
      common.zeus,
      styles.seven_segment
    )} >
      {
        score.map((digit, i) => {
          return (<Digit key={i} digit={digit}/>);
        })
      }
      <div style={{width: '18px'}} />
      {
        hiScore.map((digit, i) => {
          return (<Digit key={i} digit={digit}/>);
        })
      }
    </div>
  );
};

SevenSegment.propTypes = {
};

SevenSegment.defaultProps = {
  numberLeft: '',
  numberRight: ''
};

export default SevenSegment;
