const webpack = require('webpack');
const CopyWebpackPlugin = require('copy-webpack-plugin');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const path = require('path');

const BUILD_DIR = path.resolve(__dirname, 'dist');
const APP_DIR = path.resolve(__dirname, 'app');
const RESOURCES_DIR = path.resolve(__dirname, 'resources');
const NATIVE_DIR = path.resolve(__dirname, 'native');

module.exports = {
  entry: [
    'react-hot-loader/patch',
    'webpack-dev-server/client?http://localhost:8080',
    'webpack/hot/only-dev-server',
    path.resolve(APP_DIR, 'index.js')
  ],
  output: {
    path: BUILD_DIR,
    filename: 'bundle.js',
    publicPath: '/'
  },
  devServer: {
    hot: true,
    contentBase: BUILD_DIR,
    publicPath: '/'
  },
  mode: 'development',
  devtool: 'source-map',
  module: {
    rules: [
      {
        test: /\.jsx?/,
        loader: 'babel-loader',
        include: APP_DIR,
        options: {
          presets: [
            '@babel/env',
            '@babel/preset-react'
          ]
        }
      },
      {
        test: /\.node$/,
        loader: 'node-loader'
      },
      {
        test: /\.css$/,
        loader: 'style-loader!css-loader?importLoaders=1&modules=true&localIdentName=[name]__[local]___[hash:base64:5]'
      },
      {
        test: /.scss$/,
        loader: 'style-loader!css-loader?importLoaders=1&modules&localIdentName=[local]!sass-loader'
      },
      {
        test: /\.(png|jpg|gif)$/,
        use: 'url-loader',
        include: RESOURCES_DIR
      }
    ]
  },
  externals: {
    '../native': 'require(\'./native\')'
  },
  plugins: [
    new webpack.HotModuleReplacementPlugin(),
    new CopyWebpackPlugin([
      {
        from: './native/index.node',
        to: './native/index.node'
      }
    ]),
    new HtmlWebpackPlugin({
      template: 'resources/html/index.html',
      minify: {
        html5: true,
        removeComments: true,
        collapseWhitespace: true,
        removeRedundantAttributes: true,
        useShortDoctype: true,
        removeEmptyAttributes: true,
        removeStyleLinkTypeAttributes: true,
        keepClosingSlash: true
      },
      inject: true
    })
  ],
  target: 'electron-renderer'
};
