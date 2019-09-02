import React from 'react';
import { remote } from 'electron';

import BottomSevenSegment from './components/BottomSevenSegment';
import Screen from './components/Screen';
import ScreenDecoration from './components/ScreenDecoration';
import SevenSegment from './components/SevenSegment';
import ConsoleDecoration from './components/ConsoleDecoration';
import ConsoleLogo from './components/ConsoleLogo';
import Menu from './components/Menu';
import MenuItem from './components/MenuItem';
import Title from './components/Title';
import rustModules from '../native/index.node';

import styles from './styles.scss';

class App extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      screen: rustModules.getScreen(),
      score: [
        [false, false, false, false, false, false, false, false],
        [false, false, false, false, false, false, false, false],
        [false, false, false, false, false, false, false, false],
        [false, false, false, false, false, false, false, false],
        [false, false, false, false, false, false, false, false]
      ],
      hiScore: [
        [false, false, false, false, false, false, false, false],
        [false, false, false, false, false, false, false, false],
        [false, false, false, false, false, false, false, false],
        [false, false, false, false, false, false, false, false],
        [false, false, false, false, false, false, false, false]
      ],
      level: [
        [false, false, false, false, false, false, false, false],
        [false, false, false, false, false, false, false, false]
      ],
      speed: [
        [false, false, false, false, false, false, false, false],
        [false, false, false, false, false, false, false, false]
      ],
      loop: null
    };
  }

  openRomDialog() {
    let filename = remote.dialog.showOpenDialog({
      properties: ['openFile'],
      filters: [
        {name: 'ROM', extensions: ['zeus']}
      ]
    })[0];

    if (this.state.loop) {
      clearInterval(this.state.loop);
    }

    rustModules.loadRom(filename);
    this.setState({
      loop: setInterval(this.step.bind(this), 100)
    });
  }

  step() {
    let result = rustModules.runFrame();
    this.setState({
      screen: result.screen,
      score: result.sevenSegment.score,
      hiScore: result.sevenSegment.hiscore,
      speed: result.sevenSegment.speed,
      level: result.sevenSegment.level
    });
  }

  render() {
    return (
      <div className={styles.app_container}>
        <ConsoleDecoration />
        <ConsoleLogo />
        <Menu>
          <MenuItem onClick={this.openRomDialog.bind(this)}>
            Load cartridge
          </MenuItem>
        </Menu>
        <div className={styles.column}>
          <ScreenDecoration>
            <SevenSegment
              score={this.state.score}
              hiScore={this.state.hiScore}
            />
            <Screen
              screenData={this.state.screen}
            />
            <BottomSevenSegment
              digitsLeft={this.state.speed}
              digitsRight={this.state.level}
            />
          </ScreenDecoration>
        </div>
      </div>
    );
  }
}

export default App;
