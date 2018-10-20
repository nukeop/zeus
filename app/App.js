import React from 'react';
import { remote } from 'electron';

import Screen from './components/Screen';
import ScreenDecoration from './components/ScreenDecoration';
import Menu from './components/Menu';
import MenuItem from './components/MenuItem';
import Title from './components/Title';
import rustModules from '../native/index.node';

import styles from './styles.scss';

class App extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      screen: rustModules.getScreen()
    };
  }

  componentDidMount() {
    setInterval(this.step.bind(this), 34);
  }

  openRomDialog() {
    let filename = remote.dialog.showOpenDialog({
      properties: ['openFile'],
      filters: [
        {name: 'ROM', extensions: ['zeus']}
      ]
    })[0];
    console.log(rustModules.loadRom(filename));  
  }

  step() {
    let result = rustModules.runFrame();
    let screen = result.screen;
    this.setState({screen});
  }

  render() {
    return (
      <div className={styles.app_container}>
        <div className={styles.column}>
          <Menu>
            <MenuItem onClick={this.openRomDialog}>Load ROM</MenuItem>
            <MenuItem onClick={this.step.bind(this)}>Step</MenuItem>
          </Menu>
          <ScreenDecoration>
            <Screen
              screenData={this.state.screen}
            />
          </ScreenDecoration>
          <Title>
            9999 in 1
          </Title>
        </div>
      </div>
    );
  }
}

export default App;
