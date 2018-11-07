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
      level: [
        [false, false, false, false, false, false, false, false],
        [false, false, false, false, false, false, false, false]
      ],
      speed: [
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
              digitsLeft={this.state.speed}
              digitsRight={this.state.level}
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
