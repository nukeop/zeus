import React from 'react';
import { remote } from 'electron';

import Screen from './components/Screen';
import ScreenDecoration from './components/ScreenDecoration';
import Menu from './components/Menu';
import MenuItem from './components/MenuItem';
import Title from './components/Title';
import rustModules from '../native/index.node';

import styles from './styles.scss';

let screen = rustModules.getScreen();

const openRomDialog = () => {
  let filename = remote.dialog.showOpenDialog({
    properties: ['openFile'],
    filters: [
      {name: 'ROM', extensions: ['zeus']}
    ]
  })[0];
  console.log(rustModules.loadRom(filename));  
};

const step = () => {
  rustModules.step();
  screen = rustModules.getScreen();
};

const App = () => {
  return (
    <div className={styles.app_container}>
      <div className={styles.column}>
        <Menu>
          <MenuItem onClick={openRomDialog}>Load ROM</MenuItem>
          <MenuItem onClick={step}>Step</MenuItem>
        </Menu>
        <ScreenDecoration>
          <Screen
            screenData={screen}
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
