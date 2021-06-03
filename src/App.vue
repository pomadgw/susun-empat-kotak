<template>
  <Grid :grids="grids" />
</template>

<script lang="ts">
import { defineComponent } from 'vue'
import Grid from './components/Grid.vue'
import { create_block, put_block, create_playfield } from '../wasm-src/pkg'

export default defineComponent({
  name: 'App',
  components: {
    Grid
  },
  data: () => ({
    grids: [0],
    playfield: create_playfield(),
  }),
  mounted() {
    const block = create_block(0)
    console.log(block.to_uint8array)
    put_block(this.playfield, block, 1, 1)
    this.grids = [...this.playfield.to_uint8array]
    console.log(this.grids)
  }
})
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}

:root {
  --square-size: 20px;
}
</style>
