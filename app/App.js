import React from 'react';
import { remote } from 'electron';

import BottomSevenSegment from './components/BottomSevenSegment';
import Screen from './components/Screen';
import ScreenDecoration from './components/ScreenDecoration';
import SevenSegment from './components/SevenSegment';
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
      bottomDigitsLeft: [
        [false, false, false, false, false, false, false, false],
        [false, false, false, false, false, false, false, false]
      ],
      bottomDigitsRight: [
        [false, false, false, false, false, false, false, false],
        [false, false, false, false, false, false, false, false]
      ]
    };
  }

  openRomDialog() {
    let filename = remote.dialog.showOpenDialog({
      properties: ['openFile'],
      filters: [
        {name: 'ROM', extensions: ['zeus']}
      ]
    })[0];
    console.log(rustModules.loadRom(filename));
    setInterval(this.step.bind(this), 100);
  }

  step() {
    let result = rustModules.runFrame();
    let screen = result.screen;
    let score = result.sevenSegment.score;
    let hiScore = result.sevenSegment.hiscore;
    this.setState({
      screen,
      score,
      hiScore
    });
  }

  render() {
    return (
      <div className={styles.app_container}>
        <div className={styles.column}>
          <Menu>
            <MenuItem onClick={this.openRomDialog.bind(this)}>Load ROM</MenuItem>
            <MenuItem onClick={this.step.bind(this)}>Step</MenuItem>
          </Menu>
          <ScreenDecoration>
            <SevenSegment
              score={this.state.score}
              hiScore={this.state.hiScore}
            />
            <Screen
              screenData={this.state.screen}
            />
            <BottomSevenSegment
              digitsLeft={this.state.bottomDigitsLeft}
              digitsRight={this.state.bottomDigitsRight}
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
