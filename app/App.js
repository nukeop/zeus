import React from 'react';

import Screen from './components/Screen';
import rustModules from '../native/index.node';

import styles from './styles.scss';

const App = () => {
  return (
    <div className={styles.app_container}>
      <Screen
        screenData={rustModules.getScreen()}
      />
    </div>
  );
};

export default App;
