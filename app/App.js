import React from 'react';
import { remote } from 'electron';
import { XSound, X } from 'xsound';

import BottomSevenSegment from './components/BottomSevenSegment';
import Screen from './components/Screen';
import ScreenDecoration from './components/ScreenDecoration';
import SevenSegment from './components/SevenSegment';
import ConsoleDecoration from './components/ConsoleDecoration';
import ConsoleLogo from './components/ConsoleLogo';
import Dpad from './components/Dpad';
import Button from './components/Button';
import Menu from './components/Menu';
import MenuItem from './components/MenuItem';
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

    X('noise').type = 'pinknoise';
    X('oscillator').setup([true, true, true]);
    X('oscillator', 0).param('type', 'square');
  }

  openRomDialog() {
    let filename = remote.dialog.showOpenDialogSync({
      properties: ['openFile'],
      filters: [
        {name: 'ROM', extensions: ['zeus']}
      ]
    });
    filename = filename[0];

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
        <Dpad />
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
        <div className={styles.buttons}>
          <Button label='B' />
          <Button label='A' />
        </div>
      </div>
    );
  }
}

export default App;
