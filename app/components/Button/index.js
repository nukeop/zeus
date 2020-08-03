import React from 'react';
import cx from 'classnames';

import styles from './styles.scss';
import common from '../../styles.scss';

const Button = ({
  label
}) => (
  <div className={cx(
    common.zeus,
    styles.button
  )}>
    <button />
    <label>
      {label}
    </label>
  </div>
);

export default Button;
