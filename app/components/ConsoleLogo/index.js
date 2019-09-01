import React from 'react';
import cx from 'classnames';

import styles from './styles.scss';
import common from '../../styles.scss';
import logo from '../../../resources/images/logo.png';

const ConsoleLogo = () => {
  return (
    <div className={cx(
      common.zeus,
      styles.console_logo
    )} >
      <img src={logo}/>
    </div>
  );
};

export default ConsoleLogo;
