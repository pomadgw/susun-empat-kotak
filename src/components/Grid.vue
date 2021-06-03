<template>
  <div class="grid">
    <template v-for="i in 40" :key="`row-${i}`">
      <template v-for="j in 10" :key="`col-${j}`">
        <div :class="{ [`tcol-${j}`]: true, [`trow-${i}`]: true, color: isOn(i, j) }"></div>
      </template>
    </template>
  </div>
</template>
<script lang="ts">
import { defineComponent } from 'vue'

export default defineComponent({
  name: 'Grid',
  props: {
    grids: {
      type: Array,
      default: () => [],
    }
  },
  methods: {
    isOn(x: number, y: number) {
      // console.log(x, y, this.grids[(y - 1) * 40 + (x - 1)])
      return this.grids[(x - 1) * 10 + (y - 1)] === 1
    }
  }
})
</script>

<style scoped>
.color {
  background-color: #0ff;
}

.grid {
  display: grid;
  grid-template-columns: repeat(10, var(--square-size));
  grid-template-rows: repeat(40, var(--square-size));
}
</style>
