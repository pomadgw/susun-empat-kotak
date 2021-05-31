// import { createApp } from 'vue'
// import App from './App.vue'

import wasm, { greet } from '../wasm-src/pkg'

// createApp(App).mount('#app')

wasm().then((e) => {
  greet()
})
