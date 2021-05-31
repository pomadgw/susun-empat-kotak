<template>
  <Grid />
</template>

<script lang="ts">
import { defineComponent } from 'vue'
import Grid from './components/Grid.vue'

export default defineComponent({
  name: 'App',
  components: {
    Grid
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
