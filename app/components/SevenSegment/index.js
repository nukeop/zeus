import React from 'react';
import PropTypes from 'prop-types';
import cx from 'classnames';

import styles from './styles.scss';
import common from '../../styles.scss';

const Digit = props => {
  return (
    <div className={cx(
      common.zeus,
      styles.digit     
    )}>
      <div className={cx(styles.segment, styles.a)} />
      <div className={cx(styles.segment, styles.b)} />
      <div className={cx(styles.segment, styles.c)} />
      <div className={cx(styles.segment, styles.d)} />
      <div className={cx(styles.segment, styles.e)} />
      <div className={cx(styles.segment, styles.f)} />
      <div className={cx(styles.segment, styles.g)}>
        <div className={styles.top} />
        <div className={styles.bottom} />
      </div>
    </div>
  );
};

const SevenSegment = props => {
  return (
    <div className={cx(
      common.zeus,
      styles.seven_segment
    )} >
      <Digit digit={3} />
      <Digit digit={3} />
      <Digit digit={3} />
      <Digit digit={3} />
      <Digit digit={3} />
      <div style={{width: '18px'}} />
      <Digit digit={3} />
      <Digit digit={3} />
      <Digit digit={3} />
      <Digit digit={3} />
      <Digit digit={3} />
    </div>
  );
};

SevenSegment.propTypes = {

};

SevenSegment.defaultProps = {

};

export default SevenSegment;
