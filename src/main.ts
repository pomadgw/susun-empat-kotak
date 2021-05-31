/* eslint-disable camelcase */
import { createApp } from 'vue'
import App from './App.vue'

import initWasm, { create_block } from '../wasm-src/pkg'

initWasm().then(() => {
  for (let t = 0; t < 7; t++) {
    const m = create_block(t)
    console.log(m.toString())
    m.rotate_right()
    console.log(m.toString())
    m.rotate_right()
    console.log(m.toString())
    m.rotate_right()
    console.log(m.toString())
    m.rotate_right()
    console.log(m.toString())
  }

  createApp(App).mount('#app')
})
