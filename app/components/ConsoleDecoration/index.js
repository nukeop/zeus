import React from 'react';
import cx from 'classnames';

import styles from './styles.scss';
import common from '../../styles.scss';

const ConsoleDecoration = () => {
  return (
    <div
      className={cx(
        common.zeus,
        styles.console_decoration
      )}
    >
      <div className={cx(
        common.zeus,
        styles.stripe,
        styles.first
      )}/>
      <div className={cx(
        common.zeus,
        styles.stripe,
        styles.second
      )}/>
      <div className={cx(
        common.zeus,
        styles.stripe,
        styles.third
      )}/>
    </div>
  );
};

export default ConsoleDecoration;
