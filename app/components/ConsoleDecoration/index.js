import React from 'react';
import cx from 'classnames';

import styles from './styles.scss';
import common from '../../styles.scss';

const ConsoleDecoration = props => {
  return (
    <div
      className={cx(
        common.zeus,
        styles.console_decoration
      )}
    />
  );
};

export default ConsoleDecoration;
