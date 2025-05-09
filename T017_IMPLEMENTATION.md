# T017 Implementation: Fix webapp build issues with WebAssembly

## Overview

This implementation resolves the webpack build issues in the webapp by updating dependencies and configuring proper WebAssembly support.

## Changes Made

### 1. Updated Dependencies

Updated the following packages in `package.json`:

```json
"devDependencies": {
  "hello-wasm-pack": "^0.1.0",
  "webpack": "^5.88.2",
  "webpack-cli": "^5.1.4",
  "webpack-dev-server": "^4.15.1",
  "copy-webpack-plugin": "^11.0.0",
  "terser-webpack-plugin": "^5.3.9"
}
```

This update brings in:
- Modern webpack (v5) with better WebAssembly support
- Updated webpack-dev-server compatible with webpack 5
- Updated copy-webpack-plugin with the new pattern-based API
- Added terser-webpack-plugin for minification

### 2. Modernized webpack.config.js

Completely updated the webpack configuration to support WebAssembly:

```javascript
const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');
const TerserPlugin = require('terser-webpack-plugin');

module.exports = {
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  mode: "production",
  experiments: {
    asyncWebAssembly: true,
    syncWebAssembly: true,
  },
  optimization: {
    minimize: true,
    minimizer: [new TerserPlugin()],
  },
  plugins: [
    new CopyWebpackPlugin({
      patterns: [
        { from: "index.html" },
        { from: "manifest.json" },
        { from: "favicon.ico" },
        { from: "android-chrome-192x192.png" },
        { from: "android-chrome-512x512.png" }
      ]
    })
  ],
  module: {
    rules: [
      {
        test: /\.wasm$/,
        type: "webassembly/async"
      }
    ]
  },
  devServer: {
    static: {
      directory: path.join(__dirname, 'dist'),
    },
    compress: true,
    port: 8080,
    hot: true
  }
};
```

Key improvements:
- Added WebAssembly support through the `experiments` configuration
- Updated CopyWebpackPlugin to use the new pattern-based syntax
- Added proper module rules for .wasm files
- Configured the dev server with modern options

### 3. Updated npm Scripts

Simplified the npm scripts and removed legacy Node.js workarounds:

```json
"scripts": {
  "build": "webpack --config webpack.config.js",
  "start": "webpack serve --open",
  "dev": "webpack serve --mode development --open"
}
```

Changes:
- Removed `linuxbuild` and `linuxstart` scripts that used legacy Node.js flags
- Updated the `start` script to use `webpack serve` instead of `webpack-dev-server`
- Added a new `dev` script for development mode

## Verification Steps

1. Built the WebAssembly module:
   ```
   cd zebra_wasm && wasm-pack build
   ```

2. Installed dependencies with legacy-peer-deps (needed due to current dependency tree):
   ```
   cd zebra_webapp && npm install --legacy-peer-deps
   ```

3. Built the webapp:
   ```
   npm run build
   ```

4. Verified successful build with WebAssembly support.

## Conclusion

The webapp now builds successfully with modern webpack configuration and proper WebAssembly support. The application can now correctly use the Wasm-based line parsing functionality implemented in T011/T012.

## Future Improvements

1. Consider updating additional dependencies to avoid using `--legacy-peer-deps`
2. Add automated tests for the WebAssembly integration
3. Consider implementing a more robust error handling strategy for WebAssembly module loading
