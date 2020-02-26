const webpack = require('webpack');
const CopyWebpackPlugin = require('copy-webpack-plugin');
const path = require('path');

module.exports = {
    entry: './index.js',
    devtool: 'source-map',
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: 'index.js',
    },
    mode: 'development',
    plugins: [
        // new CopyWebpackPlugin(['index.html'])
        new webpack.IgnorePlugin(/^(wasm-worklet-processor.js|bindgen-header.js)$/),
        new CopyWebpackPlugin([
            { from: 'index.html', to: 'index.html' },
            { from: 'wasm-worklet-processor.js', to: 'wasm-worklet-processor.js' },
            { from: '../pkg', to: 'pkg' }
        ])
    ],
};
