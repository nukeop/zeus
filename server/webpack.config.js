const webpack = require('webpack');
const path = require('path');

const BUILD_DIR = path.resolve(__dirname, '..', 'dist');
const SRC_DIR = path.resolve(__dirname);

module.exports = {
  entry: path.resolve(SRC_DIR, 'index.js'),
  output: {
    path: BUILD_DIR,
    filename: 'bundle.electron.js',
    publicPath: '/'
  },
  mode: 'production',
  devtool: 'source-map',
  module: {
    rules: [
      {
        test: /\.jsx?/,
        loader: 'babel-loader',
        include: SRC_DIR,
        options: {
          presets: ['@babel/env']
        }
      }
    ]
  },  
  target: 'electron-main'
};
