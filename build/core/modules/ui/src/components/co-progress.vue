<template lang="pug">
.progress
  .bar(v-for='bar in bars' :style='{width: bar[1]}')
    span {{ bar[0] }} {{ bar[1] }}
</template>

<script lang="ts">
export default {
  name: 'progress',
  data() {
    return {
      // direction: -1, // Eventually implement default direction
      range: 200,
      ranges: [
        [0, 33], // Inidcates bar starts at 0 units and ends at 33 units
        [45, 50], // Indicates bar starts at 45 units and ends at 50 units
        ['50%'], // Indicates start at 0 position of the range, and bar is 50% of the width of the range
        [80], // Indicates start at 0 position of the range, and bar is 80 units wide
        [10, '20%'] // Indicates start at position 10, relative to the range, and the bar is to be 20% of the range
        // [24, 12], // Number order indicates direction, unless percentages are involved.. Feature TBD usefullness / how to implement
        // [-19, 123], // Numbers that exceed the range are set to the max or min and indicate that overflow has occured somehow
      ]
    }
  },
  computed: {
    bars() {
      // Create bars array to store the bars we're generating
      let bars: number[];
      // Loop through the ranges, and generate offsets and percentages
      this.ranges.forEach( element => {
        console.log(element);
      });
      return [['0%','50%']];
    }
  }
}
</script>

<style lang="scss" scoped>

.progress {
  position: relative;
  height: 2rem;
  background: #ccc;
  width: 20rem;
}
.bar {
  position: absolute;
  left: 0;
  top: 0;
  width: 0;
  background: #666;
  height: 100%;
}

</style>