import React from 'react';

import rustModules from '../native/index.node';

import styles from './styles.scss';

const App = () => {
  return (
    <div className={styles.app_container}>
      {rustModules.hello()}
    </div>
  );
};

export default App;
