import React from 'react';

import rustModules from '../native/index.node';

import styles from './styles.scss';

const App = () => {
  console.log(rustModules);
  console.log(rustModules.getMemory());
  
  return (
    <div className={styles.app_container}>
      {rustModules.hello()}
    </div>
  );
};

export default App;
