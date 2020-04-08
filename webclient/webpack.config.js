var path = require('path');
var webpack = require('webpack');

module.exports = {

    entry: './src/index.ts',
    output: {
        path: path.resolve(__dirname, 'js'),
        filename: 'index.js',
        publicPath: '/js'
    },
    module: {
        rules: [
            // {
            //     test: /\.css$/,
            //     use: [
            //         'style-loader',
            //         'css-loader'
            //     ],
            //     include: /node_modules/
            // },
            {
                test: /\.tsx?$/,
                use: [
                    'ts-loader'
                ],
                exclude: /node_modules/
            }
        ]
    },
    resolve: {
        extensions: [".tsx", ".ts", ".js"]
    },
    plugins: [
  //      new webpack.optimize.UglifyJsPlugin({
            // ...
   //     })
    ]

};