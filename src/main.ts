// import { createApp } from 'vue'
// import App from './App.vue'

import wasm, { Matrix } from '../wasm-src/pkg'

// createApp(App).mount('#app')

wasm().then((e) => {
  const m = Matrix.new(3, 3)
  m.set(0, 0, 1)
  m.set(0, 1, 2)
  m.set(0, 2, 3)
  console.log(m.to_uint8array)
  m.rotate_right()
  console.log(m.to_uint8array)
})
