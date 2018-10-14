import React from 'react';

import Screen from './components/Screen';
import ScreenDecoration from './components/ScreenDecoration';
import Menu from './components/Menu';
import MenuItem from './components/MenuItem';
import Title from './components/Title';
import rustModules from '../native/index.node';

import styles from './styles.scss';

const App = () => {
  rustModules.step();
  console.log(rustModules.getMemory());
  console.log(rustModules.getScreen());
  return (
    <div className={styles.app_container}>
      <div className={styles.column}>
        <Menu>
          <MenuItem>Load ROM</MenuItem>
        </Menu>
        <ScreenDecoration>
          <Screen
            screenData={rustModules.getScreen()}
          />
        </ScreenDecoration>
        <Title>
          9999 in 1
        </Title>
      </div>
      
    </div>
  );
};

export default App;
