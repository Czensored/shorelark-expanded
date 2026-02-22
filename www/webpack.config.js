module.exports = {
  mode: 'production',
  devtool: false,
  entry: './bootstrap-entry.js',
  output: {
    path: __dirname,
    filename: 'bootstrap.js',
    chunkFilename: 'index_js.bootstrap.js',
    webassemblyModuleFilename: 'simulation.module.wasm',
  },
  experiments: {
    asyncWebAssembly: true
  },
  resolve: {
    extensions: ['.js', '.wasm']
  },
  devServer: {
    static: {
      directory: __dirname,
    },
    open: true,
    compress: true,
    port: 8000
  }
};
