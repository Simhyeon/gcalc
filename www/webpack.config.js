const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');

module.exports = {
	entry: "./bootstrap.js",
	output: {
		path: path.resolve(__dirname, "dist"),
		filename: "bootstrap.js",
		library: 'Calculator',
		libraryTarget: 'var',
		libraryExport: 'default'
	},
	mode: "development",
	plugins: [
		new CopyWebpackPlugin({
			patterns : [ { from: './*.html' } ]
		})
	],
	experiments: {
		asyncWebAssembly: true,
	},
};
