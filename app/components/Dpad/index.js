import React from 'react';
import cx from 'classnames';

import styles from './styles.scss';
import common from '../../styles.scss';

const Dpad = () => {
  return (
    <div className={cx(
      common.zeus,
      styles.d_pad
    )} >
      <div className={styles.middle}>
        <div className={styles.bump} />
      </div>
      <div className={cx(
        styles.button,
        styles.left
      )} />
      <div className={cx(
        styles.button,
        styles.up
      )} />
      <div className={cx(
        styles.button,
        styles.right
      )} />
      <div className={cx(
        styles.button,
        styles.down
      )} />
    </div>
  );
};

export default Dpad;
