const path = require('path')
const chokidar = require('chokidar')
const { exec } = require('child_process')
const debounce = require('lodash/debounce')

const crateDir = path.resolve(__dirname, './wasm-src')
const crateSource = path.resolve(__dirname, './wasm-src')

const watcher = chokidar.watch(`${crateSource}/src/**/*.rs`, {
  ignored: /^\./,
  persistent: true
})

const compileWebm = debounce(() => {
  exec(
    'wasm-pack build --target web',
    { cwd: crateDir },
    function (error, stdout, stderr) {
      if (error) {
        console.log(error.stack)
        console.log('Error code: ' + error.code)
        console.log('Signal received: ' + error.signal)
      }
      console.log(stdout)
      console.log(stderr)
    }
  )
}, 250)

watcher
  .on('add', function (path) {
    console.log('File', path, 'has been added')
    compileWebm()
  })
  .on('change', function (path) {
    console.log('File', path, 'has been changed')
    compileWebm()
  })
  .on('unlink', function (path) {
    console.log('File', path, 'has been removed')
    compileWebm()
  })
  .on('error', function (error) {
    console.error('Error happened', error)
  })
