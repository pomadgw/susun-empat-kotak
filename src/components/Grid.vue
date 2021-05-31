<template>
  <div class="grid">
    <template v-for="i in 40" :key="`row-${i}`">
      <template v-for="j in 10" :key="`col-${j}`">
        <div :class="`trow-${i} tcol-${j}`" class="color"></div>
      </template>
    </template>
  </div>
</template>
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
